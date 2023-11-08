use faye::prelude::*;
use maud::{html, Markup};

pub fn highlight(snippet: &str) -> Markup {
    let mut classes = Vec::new();

    let mut is_fn = false;
    let mut start = 0;
    for res in Lexer::new(snippet) {
        let class = match &res {
            Ok(t) => match t.kind {
                TokenKind::Comment(_) => "faye-comment",
                TokenKind::OpenParen
                | TokenKind::CloseParen
                | TokenKind::OpenBracket
                | TokenKind::CloseBracket => "faye-bracket",
                TokenKind::Number(_) => "faye-number",
                TokenKind::Bool(_) => "faye-bool",
                TokenKind::String(_) | TokenKind::Char(_) => "faye-string",
                TokenKind::Symbol(_) if is_fn => "faye-symbol-call",
                TokenKind::Symbol(_) => "faye-symbol",
                TokenKind::Keyword(_) => "faye-keyword",
                TokenKind::Nil => "faye-nil",
            },
            Err(_) => "faye-error",
        };

        is_fn = matches!(&res, Ok(t) if t.kind == TokenKind::OpenParen);

        let end = res.map_or_else(|t| t.span, |e| e.span).bytes.end;
        classes.push((&snippet[start..end], class));
        start = end;
    }

    html! {
        @for (token, class) in classes {
            span class=(class) { (token) }
        }
    }
}
