use faye::{
    lexer::Separator,
    prelude::{
        Context, Highlighter, LexerError, LexerErrorKind, Parser, ParserError, ParserErrorKind,
        Symbol,
    },
};

pub struct FayeEditor {
    highlighter: Highlighter,
    pub ctx: Context,
}

impl FayeEditor {
    pub const fn new(highlighter: Highlighter, ctx: Context) -> Self {
        Self { highlighter, ctx }
    }
}

impl pomprt::Editor for FayeEditor {
    fn insert(&self, buffer: &mut String, cursor: &mut usize, c: char) {
        if c != ')' && c != ']' || !buffer[*cursor..].starts_with(c) {
            buffer.insert(*cursor, c);
        }

        *cursor += c.len_utf8();

        if (c == '(' || c == '[') && !buffer[*cursor..].starts_with(|c: char| !c.is_separator()) {
            buffer.insert(*cursor, if c == '(' { ')' } else { ']' });
        }
    }

    fn complete(&self, buffer: &str, cursor: usize) -> Option<pomprt::Completion> {
        let start = buffer[..cursor]
            .rfind(|c: char| c.is_separator())
            .map_or(0, |i| i + 1);
        let end = buffer[cursor..]
            .find(|c: char| c.is_separator())
            .map_or(buffer.len(), |i| cursor + i);
        let word = &buffer[start..end];
        let line = buffer[..end]
            .rsplit_once('\n')
            .map_or(&buffer[..end], |(_, line)| line);

        if !line.chars().all(|c| c.is_ascii_whitespace()) {
            let results = self
                .ctx
                .list_globals()
                .into_iter()
                .filter_map(|Symbol(s)| s.starts_with(word).then_some(s))
                .collect();

            Some(pomprt::Completion(start, end, results))
        } else {
            None
        }
    }

    fn highlight(&self, buffer: &str) -> String {
        self.highlighter.highlight(buffer)
    }

    fn highlight_prompt(&self, prompt: &str, multiline: bool) -> String {
        if multiline {
            format!("\x1b[90m{prompt}\x1b[0m")
        } else {
            format!("\x1b[36m{prompt}\x1b[0m")
        }
    }

    fn is_multiline(&self, buffer: &str, _: usize) -> bool {
        matches!(
            Parser::new(buffer).parse(),
            Err(ParserError {
                kind: ParserErrorKind::UnclosedBracket
                    | ParserErrorKind::Lexer(LexerError {
                        kind: LexerErrorKind::UnclosedChar | LexerErrorKind::UnclosedString,
                        ..
                    }),
                ..
            })
        )
    }
}
