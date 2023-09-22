// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

pub use eval::{Context, Error as EvalError, ErrorKind as EvalErrorKind, Expr};
pub use lexer::{Error as LexerError, Lexer, Location, Symbol, Token, TokenKind};
pub use parser::{Error as ParserError, ErrorKind as ParserErrorKind, Node, NodeKind, Parser};

pub mod eval;
pub mod lexer;
pub mod parser;

/// A highlighter for faye code
#[derive(Default)]
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
        let mut colors = Vec::new();

        let mut paren_idx = 0;
        let paren_colors = [
            "\x1b[92m", "\x1b[93m", "\x1b[94m", "\x1b[96m", "\x1b[95m", "\x1b[90m",
        ];

        let mut is_fn = false;
        for res in Lexer::new(snippet) {
            let color = match &res {
                Ok(Token(kind, ..)) => match kind {
                    TokenKind::Comment(_) => "\x1b[3;90m",
                    TokenKind::OpenParen | TokenKind::CloseParen if !self.match_brackets => {
                        "\x1b[90m"
                    }
                    TokenKind::OpenParen => {
                        if paren_idx > paren_colors.len() - 1 {
                            paren_idx = 0;
                        }
                        let c = paren_colors[paren_idx];
                        paren_idx += 1;
                        c
                    }
                    TokenKind::CloseParen => {
                        if paren_idx < 1 {
                            paren_idx = paren_colors.len();
                        }
                        paren_idx -= 1;
                        paren_colors[paren_idx]
                    }
                    TokenKind::Number(_) | TokenKind::Bool(_) => "\x1b[36m",
                    TokenKind::String(_) => "\x1b[33m",
                    TokenKind::Symbol(_) if is_fn => "\x1b[35m",
                    TokenKind::Symbol(_) => "\x1b[32m",
                    TokenKind::Keyword(_) => "\x1b[34m",
                    TokenKind::Nil => "\x1b[37m",
                },
                Err(_) => "\x1b[31m",
            };
            let loc = match &res {
                Ok(t) => t.1,
                Err(e) => e.start,
            };
            colors.push((loc, color));
            is_fn = matches!(res, Ok(Token(TokenKind::OpenParen, ..)));
        }

        let mut lines = snippet.split('\n').map(String::from).collect::<Vec<_>>();
        for (loc, c) in colors.iter().rev() {
            let line = &mut lines[loc.0];
            let i = line.char_indices().nth(loc.1).unwrap().0;
            line.insert_str(i, c);
        }

        lines.join("\n") + "\x1b[0m"
    }
}
