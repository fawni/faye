use crate::prelude::{Lexer, TokenKind};

/// A highlighter for faye code
#[derive(Default, Clone, Copy)]
pub struct Highlighter {
    /// Whether to highlight matching brackets or not
    match_brackets: bool,
}

impl Highlighter {
    /// Create a new highlighter and specify whether to highlight matching brackets or not
    #[must_use]
    pub const fn new(match_brackets: bool) -> Self {
        Self { match_brackets }
    }

    /// Highlight a snippet of faye code
    #[must_use]
    pub fn highlight(&self, snippet: &str) -> String {
        let mut colored = String::with_capacity(snippet.len());

        let mut paren_idx = 0;
        let paren_colors = [
            "\x1b[0;92m",
            "\x1b[0;93m",
            "\x1b[0;94m",
            "\x1b[0;96m",
            "\x1b[0;95m",
            "\x1b[0;90m",
        ];

        let mut is_fn = false;
        let mut start = 0;
        for res in Lexer::new(snippet) {
            let color = match &res {
                Ok(t) => match t.kind {
                    TokenKind::Comment(_) => "\x1b[3;90m",
                    TokenKind::OpenParen
                    | TokenKind::CloseParen
                    | TokenKind::OpenBracket
                    | TokenKind::CloseBracket
                        if !self.match_brackets =>
                    {
                        "\x1b[0;90m"
                    }
                    TokenKind::OpenParen | TokenKind::OpenBracket => {
                        if paren_idx > paren_colors.len() - 1 {
                            paren_idx = 0;
                        }
                        let c = paren_colors[paren_idx];
                        paren_idx += 1;
                        c
                    }
                    TokenKind::CloseParen | TokenKind::CloseBracket => {
                        if paren_idx < 1 {
                            paren_idx = paren_colors.len();
                        }
                        paren_idx -= 1;
                        paren_colors[paren_idx]
                    }
                    TokenKind::Number(_) => "\x1b[0;36m",
                    TokenKind::String(_) | TokenKind::Char(_) => "\x1b[0;33m",
                    TokenKind::Bool(_) | TokenKind::Nil => "\x1b[3;32m",
                    TokenKind::Symbol(_) if is_fn => "\x1b[0;35m",
                    TokenKind::Symbol(_) => "\x1b[0;37m",
                    TokenKind::Keyword(_) => "\x1b[0;34m",
                },
                Err(_) => "\x1b[31m",
            };

            is_fn = matches!(&res, Ok(t) if t.kind == TokenKind::OpenParen);

            let end = res.map_or_else(|t| t.span, |e| e.span).bytes.start;
            colored.push_str(&snippet[start..end]);
            colored.push_str(color);
            start = end;
        }

        colored.push_str(&snippet[start..]); // push any remaining whitespace
        colored.push_str("\x1b[0m");

        colored
    }
}
