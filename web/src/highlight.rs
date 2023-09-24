use faye::prelude::*;
use maud::{html, Markup};

pub fn highlight(snippet: &str) -> Markup {
    let mut classes = Vec::new();

    let mut lex = Lexer::new(snippet);

    let mut is_fn = false;
    let mut start = 0;
    while let Some(res) = lex.next() {
        let end = snippet.len() - lex.get_unparsed().len();
        let class = match &res {
            Ok(Token(kind, ..)) => match kind {
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

        is_fn = matches!(res, Ok(Token(TokenKind::OpenParen, ..)));

        classes.push((&snippet[start..end], class));
        start = end;
    }

    html! {
        @for (token, class) in classes {
            span class=(class) { (token) }
        }
    }
}
