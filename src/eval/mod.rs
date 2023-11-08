// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

pub use builtin::BuiltinFn;
pub use closure::Closure;
pub use context::Context;
pub use error::{Error, ErrorKind};
pub use expr::Expr;
pub use scope::Scope;
pub use userfn::UserFn;

mod builtin;
mod closure;
mod context;
mod error;
mod expr;
mod scope;
mod userfn;

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        ($name:ident: $input:literal, $res:expr) => {
            #[test]
            fn $name() -> Result<(), Box<dyn std::error::Error>> {
                let ast = crate::parser::Parser::new($input).parse()?;
                let res = Context::default().eval(&ast[0]);

                assert_eq!(res.map_err(|e| e.kind), $res);
                Ok(())
            }
        };
    }

    test!(mul: "(* 2 3)", Ok(Expr::Number(6.)));

    test!(error_invalid_function: "(1 + 2)", Err(
        ErrorKind::InvalidFunction(Expr::Number(1.))
    ));

    test!(error_add_string: "(+ \"hi\" 5)", Err(
        ErrorKind::InvalidArgument(Expr::String("hi".into()))
    ));
}
