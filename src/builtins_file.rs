use std::collections::HashMap;
use std::env;
use std::hash::BuildHasher;
use std::io::{self, BufReader, Read, Write};
use std::path::Path;

use glob::glob;

use crate::builtins_util::*;
use crate::environment::*;
use crate::eval::*;
use crate::interner::*;
use crate::process::*;
use crate::types::*;
use crate::unix::*;

fn cd_expand_all_dots(cd: String) -> String {
    let mut all_dots = false;
    if cd.len() > 2 {
        all_dots = true;
        for ch in cd.chars() {
            if ch != '.' {
                all_dots = false;
                break;
            }
        }
    }
    if all_dots {
        let mut new_cd = String::new();
        let paths_up = cd.len() - 2;
        new_cd.push_str("../");
        for _i in 0..paths_up {
            new_cd.push_str("../");
        }
        new_cd
    } else {
        cd
    }
}

fn builtin_cd(
    environment: &mut Environment,
    args: &mut dyn Iterator<Item = Expression>,
) -> Result<Expression, LispError> {
    let home = match env::var("HOME") {
        Ok(val) => val,
        Err(_) => "/".to_string(),
    };
    let old_dir = match env::var("OLDPWD") {
        Ok(val) => val,
        Err(_) => home.to_string(),
    };
    let new_dir = if let Some(arg) = args.next() {
        if args.next().is_none() {
            let arg_d = arg.get();
            let new_arg = match &arg_d.data {
                ExpEnum::Symbol(s, _) => match get_expression(environment, arg.clone()) {
                    Some(exp) => match &exp.get().data {
                        ExpEnum::Function(_) => eval(
                            environment,
                            Expression::alloc_data(ExpEnum::String((*s).into(), None)),
                        )?,
                        ExpEnum::Lambda(_) => eval(
                            environment,
                            Expression::alloc_data(ExpEnum::String((*s).into(), None)),
                        )?,
                        ExpEnum::Macro(_) => eval(
                            environment,
                            Expression::alloc_data(ExpEnum::String((*s).into(), None)),
                        )?,
                        _ => {
                            drop(arg_d);
                            eval(environment, &arg)?
                        }
                    },
                    _ => {
                        drop(arg_d);
                        eval(environment, &arg)?
                    }
                },
                _ => {
                    drop(arg_d);
                    eval(environment, &arg)?
                }
            }
            .as_string(environment)?;
            if let Some(h) = expand_tilde(&new_arg) {
                h
            } else {
                new_arg
            }
        } else {
            return Err(LispError::new("cd can not have more then one form"));
        }
    } else {
        home
    };
    let new_dir = if new_dir == "-" { &old_dir } else { &new_dir };
    let new_dir = cd_expand_all_dots(new_dir.to_string());
    let root = Path::new(&new_dir);
    if let Ok(oldpwd) = env::current_dir() {
        env::set_var("OLDPWD", oldpwd);
    }
    if let Err(e) = env::set_current_dir(&root) {
        eprintln!("Error changing to {}, {}", root.display(), e);
        Ok(Expression::make_nil())
    } else {
        env::set_var("PWD", env::current_dir()?);
        Ok(Expression::make_true())
    }
}

fn file_test(
    environment: &mut Environment,
    args: &mut dyn Iterator<Item = Expression>,
    test: fn(path: &Path) -> bool,
    fn_name: &str,
) -> Result<Expression, LispError> {
    if let Some(p) = args.next() {
        if args.next().is_none() {
            let p = match &eval(environment, p)?.get().data {
                ExpEnum::String(p, _) => {
                    match expand_tilde(&p) {
                        Some(p) => p,
                        None => p.to_string(), // XXX not great.
                    }
                }
                _ => {
                    let msg = format!("{} path must be a string", fn_name);
                    return Err(LispError::new(msg));
                }
            };
            let path = Path::new(&p);
            if test(path) {
                return Ok(Expression::make_true());
            } else {
                return Ok(Expression::make_nil());
            }
        }
    }
    let msg = format!("{} takes a string (a path)", fn_name);
    Err(LispError::new(msg))
}

fn builtin_path_exists(
    environment: &mut Environment,
    args: &mut dyn Iterator<Item = Expression>,
) -> Result<Expression, LispError> {
    file_test(environment, args, |path| path.exists(), "fs-exists?")
}

fn builtin_is_file(
    environment: &mut Environment,
    args: &mut dyn Iterator<Item = Expression>,
) -> Result<Expression, LispError> {
    file_test(environment, args, |path| path.is_file(), "fs-file?")
}

fn builtin_is_dir(
    environment: &mut Environment,
    args: &mut dyn Iterator<Item = Expression>,
) -> Result<Expression, LispError> {
    file_test(environment, args, |path| path.is_dir(), "fs-dir?")
}

fn pipe_write_file(pipe_in: i32, writer: &mut dyn Write) -> Result<(), LispError> {
    let mut inf = BufReader::new(fd_to_file(pipe_in));
    let mut buf = [0; 10240];
    while inf.read(&mut buf[..])? > 0 {
        writer.write_all(&mut buf[..])?;
    }
    Ok(())
}

fn builtin_pipe(
    environment: &mut Environment,
    args: &mut dyn Iterator<Item = Expression>,
) -> Result<Expression, LispError> {
    let mut pipe = args.next();
    let mut last_pid: Option<u32> = None;
    let mut pipe1 = None;
    let mut pipe2;
    let mut pipe3;
    let mut res = Ok(Expression::make_nil());
    let job = Job {
        pids: Vec::new(),
        names: Vec::new(),
        status: JobStatus::Running,
    };
    environment.jobs.borrow_mut().push(job);
    let gpo = set_grab_proc_output(environment, false);
    while let Some(p) = pipe {
        let next_pipe = args.next();
        if next_pipe.is_none() {
            // Last thing in the pipe so do not run in background.
            let old_stdin = if let Some(pipe1) = pipe1 {
                Some(replace_stdin(pipe1)?)
            } else {
                None
            };
            gpo.environment.grab_proc_output = gpo.old_grab_proc_output;
            res = eval(gpo.environment, p);
            // If pipe ended in a file then dump final output into it.
            match &res {
                Ok(res_in) => {
                    let res_d = res_in.get();
                    match &res_d.data {
                        ExpEnum::File(file) => {
                            let mut file_b = file.borrow_mut();
                            match &mut *file_b {
                                FileState::Stdout => {
                                    let stdout = io::stdout();
                                    let mut handle = stdout.lock();
                                    pipe_write_file(0, &mut handle)?;
                                }
                                FileState::Stderr => {
                                    let stderr = io::stderr();
                                    let mut handle = stderr.lock();
                                    pipe_write_file(0, &mut handle)?;
                                }
                                FileState::Write(f) => {
                                    pipe_write_file(0, f)?;
                                }
                                _ => {
                                    drop(file_b);
                                    drop(res_d);
                                    res = Err(LispError::new("File at pipe end must be writable."));
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Err(_err) => {}
            }
            if let Some(old_stdin) = old_stdin {
                dup_stdin(old_stdin)?;
            }
        } else {
            let (p1, p2) = anon_pipe()?; // p1 is read
            pipe2 = Some(p2);
            pipe3 = Some(p1);
            last_pid = Some(fork(gpo.environment, p, pipe1, pipe2)?);
            if gpo.environment.pipe_pgid.is_none() {
                gpo.environment.pipe_pgid = last_pid;
            }
            pipe1 = pipe3;
        }
        pipe = next_pipe;
    }
    gpo.environment.pipe_pgid = None;
    if res.is_err() {
        if let Some(pid) = last_pid {
            // Send a sigint to the feeding job so it does not hang on a full output buffer.
            if let Err(err) = nix::sys::signal::kill(
                nix::unistd::Pid::from_raw(pid as i32),
                nix::sys::signal::Signal::SIGINT,
            ) {
                eprintln!("ERROR, sending SIGINT to pid {}: {}", pid, err);
            }
        }
    }
    res
}

fn builtin_wait(
    environment: &mut Environment,
    args: &mut dyn Iterator<Item = Expression>,
) -> Result<Expression, LispError> {
    if let Some(arg0) = args.next() {
        if args.next().is_none() {
            let arg0 = eval(environment, arg0)?;
            return match &arg0.get().data {
                ExpEnum::Process(ProcessState::Running(pid)) => {
                    match wait_pid(environment, *pid, None) {
                        Some(exit_status) => {
                            Ok(Expression::alloc_data(ExpEnum::Int(i64::from(exit_status))))
                        }
                        None => Ok(Expression::make_nil()),
                    }
                }
                ExpEnum::Process(ProcessState::Over(_pid, exit_status)) => Ok(
                    Expression::alloc_data(ExpEnum::Int(i64::from(*exit_status))),
                ),
                ExpEnum::Int(pid) => match wait_pid(environment, *pid as u32, None) {
                    Some(exit_status) => {
                        Ok(Expression::alloc_data(ExpEnum::Int(i64::from(exit_status))))
                    }
                    None => Ok(Expression::make_nil()),
                },
                _ => Err(LispError::new("wait error: not a pid")),
            };
        }
    }
    Err(LispError::new("wait takes one form (a pid to wait on)"))
}

fn builtin_pid(
    environment: &mut Environment,
    args: &mut dyn Iterator<Item = Expression>,
) -> Result<Expression, LispError> {
    if let Some(arg0) = args.next() {
        if args.next().is_none() {
            let arg0 = eval(environment, arg0)?;
            return match arg0.get().data {
                ExpEnum::Process(ProcessState::Running(pid)) => {
                    Ok(Expression::alloc_data(ExpEnum::Int(i64::from(pid))))
                }
                ExpEnum::Process(ProcessState::Over(pid, _exit_status)) => {
                    Ok(Expression::alloc_data(ExpEnum::Int(i64::from(pid))))
                }
                _ => Err(LispError::new("pid error: not a process")),
            };
        }
    }
    Err(LispError::new("pid takes one form (a process)"))
}

fn builtin_glob(
    environment: &mut Environment,
    args: &mut dyn Iterator<Item = Expression>,
) -> Result<Expression, LispError> {
    let mut files = Vec::new();
    for pat in args {
        let pat = match &eval(environment, pat)?.get().data {
            ExpEnum::String(s, _) => s.to_string(),
            _ => return Err(LispError::new("globs need to be strings")),
        };
        let pat = match expand_tilde(&pat) {
            Some(p) => p,
            None => pat,
        };
        match glob(&pat) {
            Ok(paths) => {
                for p in paths {
                    match p {
                        Ok(p) => {
                            if let Some(p) = p.to_str() {
                                files.push(Expression::alloc_data(ExpEnum::String(
                                    p.to_string().into(),
                                    None,
                                )));
                            }
                        }
                        Err(err) => {
                            let msg = format!("glob error on while iterating {}, {}", pat, err);
                            return Err(LispError::new(msg));
                        }
                    }
                }
            }
            Err(err) => {
                let msg = format!("glob error on {}, {}", pat, err);
                return Err(LispError::new(msg));
            }
        }
    }
    Ok(Expression::with_list(files))
}

pub fn add_file_builtins<S: BuildHasher>(
    interner: &mut Interner,
    data: &mut HashMap<&'static str, (Expression, String), S>,
) {
    data.insert(
        interner.intern("cd"),
        Expression::make_function(
            builtin_cd,
            "Usage: (cd dir-to-change-to)

Change directory.

Section: shell

Example:
(mkdir \"/tmp/tst-fs-cd\")
(touch \"/tmp/tst-fs-cd/fs-cd-marker\")
(test::assert-false (fs-exists? \"fs-cd-marker\"))
(pushd \"/tmp/tst-fs-cd\")
(root::cd \"/tmp\")
(root::cd \"/tmp/tst-fs-cd\")
(test::assert-true (fs-exists? \"fs-cd-marker\"))
(rm \"/tmp/tst-fs-cd/fs-cd-marker\")
(popd)
(rmdir \"/tmp/tst-fs-cd\")
",
        ),
    );
    data.insert(
        interner.intern("fs-exists?"),
        Expression::make_function(
            builtin_path_exists,
            "Usage: (fs-exists? path-to-test)

Does the given path exist?

Section: shell

Example:
(mkdir \"/tmp/tst-fs-exists\")
(touch \"/tmp/tst-fs-exists/fs-exists\")
(test::assert-true (fs-exists? \"/tmp/tst-fs-exists/fs-exists\"))
(test::assert-true (fs-exists? \"/tmp/tst-fs-exists\"))
(test::assert-false (fs-exists? \"/tmp/tst-fs-exists/fs-exists-nope\"))
(rm \"/tmp/tst-fs-exists/fs-exists\")
(rmdir \"/tmp/tst-fs-exists\")
",
        ),
    );
    data.insert(
        interner.intern("fs-file?"),
        Expression::make_function(
            builtin_is_file,
            "Usage: (fs-file? path-to-test)

Is the given path a file?

Section: shell

Example:
(mkdir \"/tmp/tst-fs-file\")
(touch \"/tmp/tst-fs-file/fs-file\")
(test::assert-true (fs-file? \"/tmp/tst-fs-file/fs-file\"))
(test::assert-false (fs-file? \"/tmp/tst-fs-file\"))
(test::assert-false (fs-file? \"/tmp/tst-fs-file/fs-file-nope\"))
(rm \"/tmp/tst-fs-file/fs-file\")
(rmdir \"/tmp/tst-fs-file\")
",
        ),
    );
    data.insert(
        interner.intern("fs-dir?"),
        Expression::make_function(
            builtin_is_dir,
            "Usage: (fs-dir? path-to-test)

Is the given path a directory?

Section: shell

Example:
(mkdir \"/tmp/tst-fs-dir\")
(touch \"/tmp/tst-fs-dir/fs-dir-file\")
(test::assert-false (fs-dir? \"/tmp/tst-fs-dir/fs-dir-file\"))
(test::assert-true (fs-dir? \"/tmp/tst-fs-dir\"))
(test::assert-false (fs-dir? \"/tmp/tst-fs-dir/fs-dir-nope\"))
(rm \"/tmp/tst-fs-dir/fs-dir-file\")
(rmdir \"/tmp/tst-fs-dir\")
",
        ),
    );
    data.insert(
        interner.intern("pipe"),
        Expression::make_function(
            builtin_pipe,
            "Usage: (pipe (proc-whose-stdout) (is-inpup-here))

Setup a pipe between processes.

Section: shell

Example:
(def pipe-test (str (pipe (echo \"one\ntwo\nthree\")(grep two))))
(test::assert-equal \"two\n\" pipe-test)
",
        ),
    );
    data.insert(
        interner.intern("wait"),
        Expression::make_function(
            builtin_wait,
            "Usage: (wait proc-to-wait-for)

Wait for a process to end and return it's exit status.

Section: shell

Example:
(def wait-test (wait (err>null (ls /does/not/exist/123))))
(test::assert-true (> wait-test 0))
",
        ),
    );
    data.insert(
        interner.intern("pid"),
        Expression::make_function(
            builtin_pid,
            "Usage: (pid proc)

Return the pid of a process.

Section: shell

Example:
(def pid-test (echo -n))
(test::assert-true (int? (pid pid-test)))
(test::assert-error (pid 1))
",
        ),
    );
    data.insert(
        interner.intern("glob"),
        Expression::make_function(
            builtin_glob,
            "Usage: (glob /path/with/*)

Takes a list/varargs of globs and return the list of them expanded.

Section: shell

Example:
(mkdir \"/tmp/tst-fs-glob\")
(touch \"/tmp/tst-fs-glob/g1\")
(touch \"/tmp/tst-fs-glob/g2\")
(touch \"/tmp/tst-fs-glob/g3\")
(test::assert-equal '(\"/tmp/tst-fs-glob/g1\" \"/tmp/tst-fs-glob/g2\" \"/tmp/tst-fs-glob/g3\") (glob \"/tmp/tst-fs-glob/*\"))
(rm \"/tmp/tst-fs-glob/g1\")
(rm \"/tmp/tst-fs-glob/g2\")
(rm \"/tmp/tst-fs-glob/g3\")
(rmdir \"/tmp/tst-fs-glob\")
",
        ),
    );
}
