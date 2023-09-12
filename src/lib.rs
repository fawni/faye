// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

use lexer::{Lexer, Token, TokenKind};

pub mod eval;
pub mod lexer;
pub mod parser;
pub mod repl;

#[derive(Default)]
pub struct Highlighter {
    match_brackets: bool,
}

impl Highlighter {
    #[must_use]
    pub const fn new(match_brackets: bool) -> Self {
        Self { match_brackets }
    }

    #[must_use]
    pub fn highlight(&self, line: &str) -> String {
        let mut colors = Vec::new();

        let mut paren_idx = 0;
        let paren_colors = [
            "\x1b[92m", "\x1b[93m", "\x1b[94m", "\x1b[96m", "\x1b[95m", "\x1b[90m",
        ];

        let mut is_fn = false;
        for res in Lexer::new(line) {
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

        let mut lines = line.split('\n').map(String::from).collect::<Vec<_>>();
        for (loc, c) in colors.iter().rev() {
            lines[loc.0].insert_str(loc.1, c);
        }

        lines.join("\n") + "\x1b[0m"
    }
}