use faye::prelude::*;
use maud::{html, Markup};

macro_rules! err {
    ($err:ident) => {
        html! {
            (" ".repeat($err.start.1 + 2)) span.faye-error { ("^".repeat($err.end.1 - $err.start.1)) } "\n"
            span.faye-error { "error" } ": " ($err) "\n"
        }
    };
}

pub fn eval(ctx: &mut Context, expr: &str) -> Markup {
    let parser = Parser::new(expr);

    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(err) => return err!(err),
    };

    html! {
        @for n in ast {
            @match ctx.eval(&n) {
                Ok(expr) => (expr) "\n",
                Err(err) => (err!(err)),
            }
        }
    }
}
