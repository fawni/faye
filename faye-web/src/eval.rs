use faye::prelude::*;
use maud::{html, Markup};

fn display_error(span: &Span, err: &impl std::error::Error) -> Markup {
    let loc = span.location();
    let end_loc = span.end_location();

    html! {
        (" ".repeat(loc.column + 2)) span.faye-error { ("^".repeat(end_loc.column - loc.column)) } "\n"
        span.faye-error { "error" } ": " (err) "\n"
    }
}

pub fn eval(ctx: &mut Context, expr: &str) -> Markup {
    let mut parser = Parser::new(expr);

    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(err) => return display_error(&err.span, &err),
    };

    html! {
        @for n in ast {
            @match ctx.eval(&n) {
                Ok(expr) => { (expr) "\n" },
                Err(err) => (display_error(&err.span, &err)),
            }
        }
    }
}
