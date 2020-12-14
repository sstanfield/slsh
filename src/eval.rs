use std::cell::RefCell;
use std::env;
use std::sync::atomic::Ordering;

use crate::analyze::*;
use crate::builtins::{builtin_bquote, builtin_quote};
use crate::builtins_bind::{builtin_def, builtin_var};
use crate::environment::*;
use crate::gc::*;
use crate::process::*;
use crate::reader::read;
use crate::symbols::*;
use crate::types::*;

fn setup_args(
    environment: &mut Environment,
    var_names: &[&'static str],
    vars: &mut dyn Iterator<Item = Expression>,
) -> Result<(), LispError> {
    let mut names_iter = var_names.iter();
    let mut params = 0;
    loop {
        let k = names_iter.next();
        let v = vars.next();
        if k.is_none() && v.is_none() {
            break;
        } else if k.is_some() && *k.unwrap() == "&rest" {
            let rest_name = if let Some(k) = names_iter.next() {
                k
            } else {
                return Err(LispError::new("&rest requires a parameter to follow"));
            };
            if *rest_name == "&rest" {
                return Err(LispError::new("&rest can only appear once"));
            }
            if names_iter.next().is_some() {
                return Err(LispError::new("&rest must be before the last parameter"));
            }
            let mut rest_data: Vec<Handle> = Vec::new();
            if let Some(v) = v {
                rest_data.push(v.into());
            }
            for v in vars {
                rest_data.push(v.into());
            }
            if rest_data.is_empty() {
                environment
                    .stack
                    .push(Binding::with_expression(Expression::make_nil()));
            } else {
                environment
                    .stack
                    .push(Binding::with_expression(Expression::with_list(rest_data)));
            }
            return Ok(());
        } else if k.is_none() || v.is_none() {
            let mut min_params = params;
            if v.is_some() {
                params += 1;
            }
            if k.is_some() {
                min_params += 1;
            }
            let mut has_rest = false;
            for k in names_iter {
                if *k == "&rest" {
                    has_rest = true;
                } else {
                    min_params += 1;
                }
            }
            let msg = if has_rest {
                format!(
                    "wrong number of parameters, expected at least {} got {}",
                    min_params,
                    (params + vars.count())
                )
            } else {
                format!(
                    "wrong number of parameters, expected {} got {} [{:?}]",
                    min_params,
                    (params + vars.count()),
                    var_names
                )
            };
            return Err(LispError::new(msg));
        }
        let var = v.unwrap().resolve(environment)?;
        environment.stack.push(Binding::with_expression(var));
        params += 1;
    }
    Ok(())
}

fn prep_stack(
    environment: &mut Environment,
    vars: &mut dyn Iterator<Item = Expression>,
    lambda: &Lambda,
    lambda_exp: Expression,
) -> Result<(), LispError> {
    let index = environment.stack.len();
    setup_args(environment, &lambda.params, vars)?;
    let symbols = lambda.syms.clone();
    // Push the 'this-fn' value.
    environment.stack.push(Binding::with_expression(lambda_exp));
    let mut i = 0;
    let extras = symbols.len() - (environment.stack.len() - index);
    while i < extras {
        environment.stack.push(Binding::new());
        i += 1;
    }
    symbols.stack_captures(environment, index);
    environment.stack_frames.push(StackFrame { index, symbols });
    environment.stack_frame_base = index;
    Ok(())
}

fn call_lambda_int(
    environment: &mut Environment,
    lambda_exp: Expression,
    lambda: Lambda,
    args: &mut dyn Iterator<Item = Expression>,
    eval_args: bool,
) -> Result<Expression, LispError> {
    let mut lambda_int = lambda;
    let mut lambda: &mut Lambda = &mut lambda_int;
    let mut body: Expression = lambda.body.clone_root().into();
    let stack_len = environment.stack.len();
    let stack_frames_len = environment.stack_frames.len();
    let stack_base = environment.stack_frame_base;
    let mut lambda_current = lambda_exp;
    if eval_args {
        let mut tvars: Vec<Handle> = Vec::new();
        for v in args {
            tvars.push(eval(environment, &v)?.into());
        }
        prep_stack(
            environment,
            &mut box_slice_it(&tvars),
            &lambda,
            lambda_current.clone(),
        )?;
    } else {
        prep_stack(environment, args, &lambda, lambda_current.clone())?;
    }

    let mut llast_eval: Option<Expression> = None;
    let mut looping = true;
    while looping {
        if environment.sig_int.load(Ordering::Relaxed) {
            environment.sig_int.store(false, Ordering::Relaxed);
            return Err(LispError::new("Lambda interupted by SIGINT."));
        }
        let last_eval = eval_nr(environment, &body)?;
        looping = environment.state.recur_num_args.is_some() && environment.exit_code.is_none();
        if looping {
            // This is a recur call, must be a tail call.
            let recur_args = environment.state.recur_num_args.unwrap();
            environment.state.recur_num_args = None;
            if let ExpEnum::Vector(new_args) = &last_eval.get().data {
                if recur_args != new_args.len() {
                    return Err(LispError::new("Called recur in a non-tail position."));
                }
                environment.stack.truncate(stack_len);
                environment.stack_frames.truncate(stack_frames_len);
                environment.stack_frame_base = stack_base;
                prep_stack(
                    environment,
                    &mut ListIter::new_list(&new_args),
                    &lambda,
                    lambda_current.clone(),
                )?;
            }
        } else if environment.exit_code.is_none() {
            // This will detect a normal tail call and optimize it.
            if let ExpEnum::LazyFn(lam, parts) = &last_eval.get().data {
                lambda_current = lam.into();
                let lam_d = lambda_current.get();
                if let ExpEnum::Lambda(lam) = &lam_d.data {
                    lambda_int = lam.clone();
                    drop(lam_d);
                    lambda = &mut lambda_int;
                    body = lambda.body.clone_root().into();
                    looping = true;
                    environment.namespace = lambda.syms.namespace().clone();
                    environment.stack.truncate(stack_len);
                    environment.stack_frames.truncate(stack_frames_len);
                    environment.stack_frame_base = stack_base;
                    prep_stack(
                        environment,
                        &mut ListIter::new_list(&parts),
                        &lambda,
                        lambda_current.clone(),
                    )?;
                }
            }
        }
        llast_eval = Some(last_eval);
    }
    Ok(llast_eval
        .unwrap_or_else(Expression::make_nil)
        .resolve(environment)?)
}

pub fn call_lambda(
    environment: &mut Environment,
    lambda_exp: Expression,
    args: &mut dyn Iterator<Item = Expression>,
    eval_args: bool,
) -> Result<Expression, LispError> {
    let lambda = if let ExpEnum::Lambda(l) = &lambda_exp.get().data {
        l.clone()
    } else if let ExpEnum::Macro(l) = &lambda_exp.get().data {
        l.clone()
    } else {
        return Err(LispError::new(format!(
            "Lambda required got {} {}.",
            lambda_exp.display_type(),
            lambda_exp
        )));
    };
    let old_ns = environment.namespace.clone();
    environment.namespace = lambda.syms.namespace().clone();
    let old_loose = environment.loose_symbols;
    let stack_len = environment.stack.len();
    let stack_frames_len = environment.stack_frames.len();
    let old_base = environment.stack_frame_base;
    environment.loose_symbols = false;
    let ret = call_lambda_int(environment, lambda_exp, lambda, args, eval_args);
    environment.loose_symbols = old_loose;
    environment.stack.truncate(stack_len);
    environment.stack_frames.truncate(stack_frames_len);
    environment.stack_frame_base = old_base;
    environment.namespace = old_ns;
    ret
}

fn exec_macro(
    environment: &mut Environment,
    sh_macro: &Lambda,
    args: &mut dyn Iterator<Item = Expression>,
) -> Result<Expression, LispError> {
    //let bb: Expression = sh_macro.body.clone().into();
    //println!("XXXX exec macro for {}", bb);
    let expansion = call_lambda(
        environment,
        ExpEnum::Lambda(sh_macro.clone()).into(),
        args,
        false,
    )?
    .resolve(environment)?;
    //println!("XXXX execed macro for ");
    let last_frame = environment.stack_frames.last();
    if let Some(frame) = last_frame {
        let mut syms = Some(frame.symbols.clone());
        analyze(environment, &expansion, &mut syms)?;
    }
    eval(environment, &expansion)
}

fn make_lazy(
    environment: &mut Environment,
    lambda: Expression,
    args: &mut dyn Iterator<Item = Expression>,
) -> Result<Expression, LispError> {
    let mut parms: Vec<Handle> = Vec::new();
    for p in args {
        parms.push(eval(environment, p)?.into());
    }
    Ok(Expression::alloc(ExpObj {
        data: ExpEnum::LazyFn(lambda.into(), parms),
        meta: None,
        meta_tags: None,
        analyzed: RefCell::new(true),
    }))
}

pub fn box_slice_it<'a>(v: &'a [Handle]) -> Box<dyn Iterator<Item = Expression> + 'a> {
    Box::new(ListIter::new_slice(v))
}

fn eval_command(
    environment: &mut Environment,
    com_exp: &Expression,
    parts: &mut dyn Iterator<Item = Expression>,
) -> Result<Expression, LispError> {
    let allow_sys_com =
        environment.form_type == FormType::ExternalOnly || environment.form_type == FormType::Any;
    let com_exp_d = com_exp.get();
    match &com_exp_d.data {
        ExpEnum::Lambda(_) => {
            drop(com_exp_d);
            if environment.allow_lazy_fn {
                make_lazy(environment, com_exp.clone(), parts)
            } else {
                call_lambda(environment, com_exp.clone(), parts, true)
            }
        }
        ExpEnum::Macro(m) => exec_macro(environment, &m, parts),
        ExpEnum::Function(c) => (c.func)(environment, &mut *parts),
        ExpEnum::DeclareDef => builtin_def(environment, &mut *parts),
        ExpEnum::DeclareVar => builtin_var(environment, &mut *parts),
        ExpEnum::Quote => builtin_quote(environment, &mut *parts),
        ExpEnum::BackQuote => builtin_bquote(environment, &mut *parts),
        ExpEnum::String(s, _) if allow_sys_com => do_command(environment, s.trim(), parts),
        _ => {
            let msg = format!(
                "Not a valid command {}, type {}.",
                com_exp,
                com_exp.display_type()
            );
            Err(LispError::new(msg))
        }
    }
}

fn fn_eval_lazy(
    environment: &mut Environment,
    expression: &Expression,
) -> Result<Expression, LispError> {
    let exp_d = expression.get();
    let e2: Expression;
    let e2_d;
    let (command, mut parts) = match &exp_d.data {
        ExpEnum::Vector(parts) => {
            let (command, parts) = match parts.split_first() {
                Some((c, p)) => (c, p),
                None => {
                    return Err(LispError::new("No valid command."));
                }
            };
            let ib = box_slice_it(parts);
            (command.clone(), ib)
        }
        ExpEnum::Pair(e1, ie2) => {
            e2 = ie2.into();
            e2_d = e2.get();
            let e2_iter = if let ExpEnum::Vector(list) = &e2_d.data {
                Box::new(ListIter::new_list(&list))
            } else {
                drop(e2_d);
                e2.iter()
            };
            (e1.clone(), e2_iter)
        }
        ExpEnum::Nil => return Ok(Expression::alloc_data(ExpEnum::Nil)),
        _ => return Err(LispError::new("Not a callable expression.")),
    };
    let command: Expression = command.into();
    let command = command.resolve(environment)?;
    let command_d = command.get();
    let allow_sys_com =
        environment.form_type == FormType::ExternalOnly || environment.form_type == FormType::Any;
    let allow_form =
        environment.form_type == FormType::FormOnly || environment.form_type == FormType::Any;
    match &command_d.data {
        ExpEnum::Symbol(command_sym, _) => {
            if command_sym.is_empty() {
                return Ok(Expression::alloc_data(ExpEnum::Nil));
            }
            //let command_sym = <&str>::clone(command_sym); // XXX this sucks, try to work around the drop below...
            let command_sym: &'static str = command_sym; // This makes the drop happy.
            drop(command_d);
            let form = get_expression(environment, command.clone());
            if let Some(exp) = form {
                match &exp.get().data {
                    ExpEnum::Function(c) if allow_form => (c.func)(environment, &mut parts),
                    ExpEnum::DeclareDef if allow_form => builtin_def(environment, &mut parts),
                    ExpEnum::DeclareVar if allow_form => builtin_var(environment, &mut parts),
                    ExpEnum::Quote if allow_form => builtin_quote(environment, &mut *parts),
                    ExpEnum::BackQuote if allow_form => builtin_bquote(environment, &mut *parts),
                    ExpEnum::Lambda(_) if allow_form => {
                        if environment.allow_lazy_fn {
                            make_lazy(environment, exp.clone(), &mut parts)
                        } else {
                            call_lambda(environment, exp.clone(), &mut parts, true)
                        }
                    }
                    ExpEnum::Macro(m) if allow_form => exec_macro(environment, &m, &mut parts),
                    ExpEnum::String(s, _) if allow_sys_com => {
                        do_command(environment, s.trim(), &mut parts)
                    }
                    _ => {
                        if command_sym.starts_with('$') {
                            if let ExpEnum::String(command_sym, _) =
                                &str_process(environment, command_sym, true)?.get().data
                            {
                                do_command(environment, &command_sym, &mut parts)
                            } else {
                                let msg = format!("Not a valid form {}, not found.", command_sym);
                                Err(LispError::new(msg))
                            }
                        } else {
                            do_command(environment, command_sym, &mut parts)
                        }
                    }
                }
            } else if allow_sys_com {
                if command_sym.starts_with('$') {
                    if let ExpEnum::String(command_sym, _) =
                        &str_process(environment, command_sym, true)?.get().data
                    {
                        do_command(environment, &command_sym, &mut parts)
                    } else {
                        let msg = format!("Not a valid form {}, not found.", command_sym);
                        Err(LispError::new(msg))
                    }
                } else {
                    do_command(environment, command_sym, &mut parts)
                }
            } else {
                let msg = format!("Not a valid form {}, not found.", command);
                Err(LispError::new(msg))
            }
        }
        ExpEnum::Vector(_) => {
            drop(command_d); // Drop the lock on command.
            let com_exp = eval(environment, &command)?;
            eval_command(environment, &com_exp, &mut parts)
        }
        ExpEnum::Pair(_, _) => {
            drop(command_d); // Drop the lock on command.
            let com_exp = eval(environment, &command)?;
            eval_command(environment, &com_exp, &mut parts)
        }
        ExpEnum::Lambda(_) => {
            if environment.allow_lazy_fn {
                make_lazy(environment, command.clone(), &mut parts)
            } else {
                call_lambda(environment, command.clone(), &mut parts, true)
            }
        }
        ExpEnum::Macro(m) => exec_macro(environment, &m, &mut parts),
        ExpEnum::Function(c) => (c.func)(environment, &mut *parts),
        ExpEnum::DeclareDef => builtin_def(environment, &mut *parts),
        ExpEnum::DeclareVar => builtin_var(environment, &mut *parts),
        ExpEnum::Quote => builtin_quote(environment, &mut *parts),
        ExpEnum::BackQuote => builtin_bquote(environment, &mut *parts),
        ExpEnum::String(s, _) if allow_sys_com => do_command(environment, s.trim(), &mut parts),
        ExpEnum::Wrapper(_) => {
            drop(command_d); // Drop the lock on command.
            let com_exp = eval(environment, &command)?;
            eval_command(environment, &com_exp, &mut parts)
        }
        _ => {
            let msg = format!(
                "Not a valid command {}, type {}.",
                command.make_string(environment)?,
                command.display_type()
            );
            Err(LispError::new(msg))
        }
    }
}

fn str_process(
    environment: &mut Environment,
    string: &str,
    expand: bool,
) -> Result<Expression, LispError> {
    if expand && !environment.str_ignore_expand && string.contains('$') {
        let mut new_string = String::new();
        let mut last_ch = '\0';
        let mut in_var = false;
        let mut in_command = false;
        let mut command_depth: i32 = 0;
        let mut var_start = 0;
        for (i, ch) in string.chars().enumerate() {
            if in_var {
                if ch == '(' && var_start + 1 == i {
                    in_command = true;
                    in_var = false;
                    command_depth = 1;
                } else {
                    if ch == ' ' || ch == '"' || ch == ':' || (ch == '$' && last_ch != '\\') {
                        in_var = false;
                        match env::var(&string[var_start + 1..i]) {
                            Ok(val) => new_string.push_str(&val),
                            Err(_) => new_string.push_str(""),
                        }
                    }
                    if ch == ' ' || ch == '"' || ch == ':' {
                        new_string.push(ch);
                    }
                }
            } else if in_command {
                if ch == ')' && last_ch != '\\' {
                    command_depth -= 1;
                }
                if command_depth == 0 {
                    in_command = false;
                    let ast = read(environment, &string[var_start + 1..=i], None, false);
                    match ast {
                        Ok(ast) => {
                            environment.loose_symbols = true;
                            let old_out = environment.state.stdout_status.clone();
                            let old_err = environment.state.stderr_status.clone();
                            environment.state.stdout_status = Some(IOState::Pipe);
                            environment.state.stderr_status = Some(IOState::Pipe);

                            // Get out of a pipe for the str call if in one...
                            let data_in = environment.data_in.clone();
                            environment.data_in = None;
                            let in_pipe = environment.in_pipe;
                            environment.in_pipe = false;
                            let pipe_pgid = environment.state.pipe_pgid;
                            environment.state.pipe_pgid = None;
                            new_string.push_str(
                                eval(environment, ast)
                                    .map_err(|e| {
                                        environment.state.stdout_status = old_out.clone();
                                        environment.state.stderr_status = old_err.clone();
                                        e
                                    })?
                                    .as_string(environment)
                                    .map_err(|e| {
                                        environment.state.stdout_status = old_out.clone();
                                        environment.state.stderr_status = old_err.clone();
                                        e
                                    })?
                                    .trim(),
                            );
                            environment.state.stdout_status = old_out;
                            environment.state.stderr_status = old_err;
                            environment.data_in = data_in;
                            environment.in_pipe = in_pipe;
                            environment.state.pipe_pgid = pipe_pgid;
                            environment.loose_symbols = false;
                        }
                        Err(err) => return Err(LispError::new(err.reason)),
                    }
                } else if ch == '(' && last_ch != '\\' {
                    command_depth += 1;
                }
            } else if ch == '$' && last_ch != '\\' {
                in_var = true;
                var_start = i;
            } else if ch != '\\' {
                if last_ch == '\\' && ch != '$' {
                    new_string.push('\\');
                }
                new_string.push(ch);
            }
            last_ch = ch;
        }
        if in_var {
            match env::var(&string[var_start + 1..]) {
                Ok(val) => new_string.push_str(&val),
                Err(_) => new_string.push_str(""),
            }
        }
        if in_command {
            return Err(LispError::new(
                "Malformed command embedded in string (missing ')'?).",
            ));
        }
        if environment.interner.contains(&new_string) {
            Ok(Expression::alloc_data(ExpEnum::String(
                environment.interner.intern(&new_string).into(),
                None,
            )))
        } else {
            Ok(Expression::alloc_data(ExpEnum::String(
                new_string.into(),
                None,
            )))
        }
    } else if environment.interner.contains(string) {
        Ok(Expression::alloc_data(ExpEnum::String(
            environment.interner.intern(string).into(),
            None,
        )))
    } else {
        Ok(Expression::alloc_data(ExpEnum::String(
            string.to_string().into(),
            None,
        )))
    }
}

fn internal_eval(
    environment: &mut Environment,
    expression_in: &Expression,
) -> Result<Expression, LispError> {
    let expression = expression_in.clone_root();
    if environment.sig_int.load(Ordering::Relaxed) {
        environment.sig_int.store(false, Ordering::Relaxed);
        return Err(LispError::new("Script interupted by SIGINT."));
    }
    // exit was called so just return nil to unwind.
    if environment.exit_code.is_some() {
        return Ok(Expression::alloc_data(ExpEnum::Nil));
    }
    let in_recur = environment.state.recur_num_args.is_some();
    if in_recur {
        environment.state.recur_num_args = None;
        return Err(LispError::new("Called recur in a non-tail position."));
    }
    let exp_a = expression.get();
    let exp_d = &exp_a.data;
    let ret = match exp_d {
        ExpEnum::Vector(_) => {
            drop(exp_a);
            environment.last_meta = expression.meta();
            let ret = fn_eval_lazy(environment, &expression)?;
            Ok(ret)
        }
        ExpEnum::Values(v) => {
            if v.is_empty() {
                Ok(Expression::make_nil())
            } else {
                let v: Expression = (&v[0]).into();
                internal_eval(environment, &v)
            }
        }
        ExpEnum::Pair(_, _) => {
            drop(exp_a);
            environment.last_meta = expression.meta();
            let ret = fn_eval_lazy(environment, &expression)?;
            Ok(ret)
        }
        ExpEnum::Nil => Ok(expression.clone()),
        ExpEnum::Symbol(sym, SymLoc::Ref(binding)) => {
            if let Some(reference) = environment.dynamic_scope.get(sym) {
                Ok(reference.get())
            } else {
                Ok(binding.get())
            }
        }
        ExpEnum::Symbol(sym, SymLoc::Namespace(scope, idx)) => {
            if let Some(exp) = scope.borrow().get_idx(*idx) {
                Ok(exp)
            } else {
                Err(LispError::new(format!(
                    "Symbol {} not found in namespace {}.",
                    sym,
                    scope.borrow().name()
                )))
            }
        }
        ExpEnum::Symbol(_, SymLoc::Stack(idx)) => {
            if let Some(exp) = get_expression_stack(environment, *idx) {
                Ok(exp)
            } else {
                panic!("Invalid stack reference!");
            }
        }
        ExpEnum::Symbol(s, SymLoc::None) => {
            if s.starts_with('$') {
                match env::var(&s[1..]) {
                    Ok(val) => Ok(Expression::alloc_data(ExpEnum::String(
                        environment.interner.intern(&val).into(),
                        None,
                    ))),
                    Err(_) => Ok(Expression::alloc_data(ExpEnum::Nil)),
                }
            } else if s.starts_with(':') {
                // Got a keyword, so just be you...
                Ok(Expression::alloc_data(ExpEnum::Symbol(s, SymLoc::None)))
            } else if let Some(exp) = get_expression(environment, expression.clone()) {
                let exp_d = exp.get();
                if let ExpEnum::Symbol(sym, _) = &exp_d.data {
                    Ok(ExpEnum::Symbol(sym, SymLoc::None).into()) // XXX TODO- better copy.
                } else {
                    drop(exp_d);
                    Ok(exp)
                }
            } else if environment.loose_symbols {
                str_process(environment, s, false)
            } else {
                let msg = format!("Symbol {} not found x.", s);
                Err(LispError::new(msg))
            }
        }
        ExpEnum::HashMap(_) => Ok(expression.clone()),
        // If we have an iterator on the string then assume it is already processed and being used.
        // XXX TODO- verify this assumption is correct, maybe change when to process strings.
        ExpEnum::String(_, Some(_)) => Ok(expression.clone()),
        ExpEnum::String(string, _) => str_process(environment, &string, true),
        ExpEnum::True => Ok(expression.clone()),
        ExpEnum::Float(_) => Ok(expression.clone()),
        ExpEnum::Int(_) => Ok(expression.clone()),
        ExpEnum::Char(_) => Ok(expression.clone()),
        ExpEnum::CodePoint(_) => Ok(expression.clone()),
        ExpEnum::Lambda(_) => Ok(expression.clone()),
        ExpEnum::Macro(_) => Ok(expression.clone()),
        ExpEnum::Function(_) => Ok(Expression::alloc_data(ExpEnum::Nil)),
        ExpEnum::DeclareDef => Ok(Expression::alloc_data(ExpEnum::Nil)),
        ExpEnum::DeclareVar => Ok(Expression::alloc_data(ExpEnum::Nil)),
        ExpEnum::Quote => Ok(Expression::alloc_data(ExpEnum::Nil)),
        ExpEnum::BackQuote => Ok(Expression::alloc_data(ExpEnum::Nil)),
        ExpEnum::Process(_) => Ok(expression.clone()),
        ExpEnum::File(_) => Ok(Expression::alloc_data(ExpEnum::Nil)),
        ExpEnum::LazyFn(_, _) => {
            let int_exp = expression.clone().resolve(environment)?;
            eval(environment, int_exp)
        }
        ExpEnum::Wrapper(exp) => {
            let exp: Expression = exp.into();
            let exp_d = exp.get();
            match &exp_d.data {
                ExpEnum::Lambda(l) => {
                    let p = l.params.clone();
                    let mut syms = l.syms.dup();
                    syms.refresh_captures(environment)?;
                    Ok(Expression::alloc_data(ExpEnum::Lambda(Lambda {
                        params: p,
                        body: l.body.clone(),
                        syms,
                        namespace: environment.namespace.clone(),
                    })))
                }
                ExpEnum::Macro(l) => {
                    let p = l.params.clone();
                    let mut syms = l.syms.dup();
                    syms.refresh_captures(environment)?;
                    Ok(Expression::alloc_data(ExpEnum::Macro(Lambda {
                        params: p,
                        body: l.body.clone(),
                        syms,
                        namespace: environment.namespace.clone(),
                    })))
                }
                _ => {
                    drop(exp_d);
                    Ok(exp)
                }
            }
        }
        ExpEnum::DeclareFn => panic!("Illegal fn state in eval, was analyze skipped?"),
        ExpEnum::DeclareMacro => panic!("Illegal fn state in eval, was analyze skipped?"),
        ExpEnum::Undefined => {
            panic!("Illegal fn state in eval, tried to eval an undefined symbol!")
        }
    };
    ret
}

pub fn eval_nr(
    environment: &mut Environment,
    expression: impl AsRef<Expression>,
) -> Result<Expression, LispError> {
    let expression = expression.as_ref();
    if environment.supress_eval {
        return Ok(expression.clone());
    }
    if environment.return_val.is_some() {
        return Ok(Expression::alloc_data(ExpEnum::Nil));
    }
    if environment.state.eval_level > 500 {
        return Err(LispError::new("Eval calls to deep."));
    }
    environment.state.eval_level += 1;
    analyze(environment, expression, &mut None)?;
    let tres = internal_eval(environment, &expression);
    let mut result = if environment.state.eval_level == 1 && environment.return_val.is_some() {
        environment.return_val = None;
        Err(LispError::new("Return without matching block."))
    } else {
        tres
    };
    if let Err(err) = &mut result {
        if err.backtrace.is_none() {
            err.backtrace = Some(Vec::new());
        }
        if let Some(backtrace) = &mut err.backtrace {
            backtrace.push(expression.clone().into());
        }
    }
    environment.state.eval_level -= 1;
    environment.last_meta = None;
    result
}

pub fn eval(
    environment: &mut Environment,
    expression: impl AsRef<Expression>,
) -> Result<Expression, LispError> {
    let expression = expression.as_ref();
    eval_nr(environment, expression)?.resolve(environment)
}

pub fn eval_data(environment: &mut Environment, data: ExpEnum) -> Result<Expression, LispError> {
    let data = Expression::alloc_data(data);
    eval(environment, data)
}

pub fn eval_no_values(
    environment: &mut Environment,
    expression: impl AsRef<Expression>,
) -> Result<Expression, LispError> {
    let expression = expression.as_ref();
    let exp = eval(environment, expression)?;
    let exp_d = exp.get();
    if let ExpEnum::Values(v) = &exp_d.data {
        if v.is_empty() {
            Ok(Expression::make_nil())
        } else {
            Ok((&v[0]).into())
        }
    } else {
        drop(exp_d);
        Ok(exp)
    }
}
