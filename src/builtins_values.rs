use std::collections::HashMap;
use std::hash::BuildHasher;
use std::io;

use crate::environment::*;
use crate::eval::*;
use crate::gc::Handle;
use crate::interner::*;
use crate::types::*;

fn builtin_values(
    environment: &mut Environment,
    args: &mut dyn Iterator<Item = Expression>,
) -> io::Result<Expression> {
    let mut vals: Vec<Handle> = Vec::new();
    for a in args {
        vals.push(eval(environment, a)?.handle_no_root());
    }
    Ok(Expression::alloc_data(ExpEnum::Values(vals)))
}

fn builtin_values_nth(
    environment: &mut Environment,
    args: &mut dyn Iterator<Item = Expression>,
) -> io::Result<Expression> {
    if let Some(idx) = args.next() {
        if let Some(vals) = args.next() {
            if args.next().is_none() {
                if let ExpEnum::Atom(Atom::Int(idx)) = &eval(environment, idx)?.get().data {
                    if let ExpEnum::Values(vals) = &eval(environment, vals)?.get().data {
                        if *idx < 0 || *idx >= vals.len() as i64 {
                            let msg =
                                format!("values-nth index {} out of range {}", idx, vals.len());
                            return Err(io::Error::new(io::ErrorKind::Other, msg));
                        }
                        return Ok(vals[*idx as usize].clone().into());
                    }
                }
            }
        }
    }
    Err(io::Error::new(
        io::ErrorKind::Other,
        "values-nth takes two forms (int and multi values object)",
    ))
}

fn builtin_values_length(
    environment: &mut Environment,
    args: &mut dyn Iterator<Item = Expression>,
) -> io::Result<Expression> {
    if let Some(vals) = args.next() {
        if args.next().is_none() {
            if let ExpEnum::Values(vals) = &eval(environment, vals)?.get().data {
                return Ok(Expression::alloc_data(ExpEnum::Atom(Atom::Int(
                    vals.len() as i64
                ))));
            }
        }
    }
    Err(io::Error::new(
        io::ErrorKind::Other,
        "values-length takes one form- a muti values object",
    ))
}

pub fn add_values_builtins<S: BuildHasher>(
    interner: &mut Interner,
    data: &mut HashMap<&'static str, Reference, S>,
) {
    let root = interner.intern("root");
    data.insert(
        interner.intern("values"),
        Expression::make_function(
            builtin_values,
            "Usage: (values expression*)

Produces a multi values object.  Useful for returning more then one value from
a function when most of time you only care about the first (primary) item.  When
evaluting a muti values object it will evaluate as if it the first item only.

Section: root

Example:
(test::assert-true (values? (values 1 \"str\" 5.5)))
(test::assert-equal 1 (values-nth 0 (values 1 \"str\" 5.5)))
(test::assert-equal \"str\" (values-nth 1 (values 1 \"str\" 5.5)))
(test::assert-equal 5.5 (values-nth 2 (values 1 \"str\" 5.5)))
",
            root,
        ),
    );
    data.insert(
        interner.intern("values-nth"),
        Expression::make_function(
            builtin_values_nth,
            "Usage: (values-nth idx expression)

If expression is a values object then return the item at index idx.

Section: root

Example:
(test::assert-equal 1 (values-nth 0 (values 1 \"str\" 5.5)))
(test::assert-equal \"str\" (values-nth 1 (values 1 \"str\" 5.5)))
(test::assert-equal 5.5 (values-nth 2 (values 1 \"str\" 5.5)))
(def 'test-vals-nth (values 1 \"str\" 5.5))
(test::assert-equal 1 (values-nth 0 test-vals-nth))
(test::assert-equal \"str\" (values-nth 1 test-vals-nth))
(test::assert-equal 5.5 (values-nth 2 test-vals-nth))
",
            root,
        ),
    );
    data.insert(
        interner.intern("values-length"),
        Expression::make_function(
            builtin_values_length,
            "Usage: (values-length expression)

If expression is a values object then return it's length (number of values).

Section: root

Example:
(test::assert-equal 3 (values-length (values 1 \"str\" 5.5)))
(test::assert-equal 2 (values-length (values 1 \"str\")))
(test::assert-equal 1 (values-length (values \"str\")))
(test::assert-equal 0 (values-length (values)))
(test::assert-equal \"str\" (values-nth 1 (values 1 \"str\" 5.5)))
(test::assert-equal 5.5 (values-nth 2 (values 1 \"str\" 5.5)))
(def 'test-vals-len (values 1 \"str\" 5.5))
(test::assert-equal 3 (values-length test-vals-len))
",
            root,
        ),
    );
}
