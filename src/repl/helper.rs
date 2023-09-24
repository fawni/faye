use faye::{Highlighter, Parser, ParserErrorKind};
use rustyline::validate;

/// A rustyline helper for faye
pub struct FayeHelper {
    highlighter: Highlighter,
}

impl FayeHelper {
    pub const fn new(highlighter: Highlighter) -> Self {
        Self { highlighter }
    }
}

impl rustyline::Helper for FayeHelper {}

impl rustyline::completion::Completer for FayeHelper {
    type Candidate = String;
}

impl rustyline::hint::Hinter for FayeHelper {
    type Hint = String;
    fn hint(&self, line: &str, a: usize, b: &rustyline::Context) -> Option<Self::Hint> {
        let hinter = rustyline::hint::HistoryHinter {};
        hinter.hint(line, a, b)
    }
}

impl rustyline::highlight::Highlighter for FayeHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        _default: bool,
    ) -> std::borrow::Cow<'b, str> {
        format!("\x1b[36m{prompt}\x1b[0m").into()
    }

    fn highlight_char(&self, _line: &str, _cursor_pos: usize) -> bool {
        true
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> std::borrow::Cow<'h, str> {
        format!("\x1b[3;90m{hint}\x1b[0m").into()
    }

    fn highlight<'l>(&self, line: &'l str, _cursor_pos: usize) -> std::borrow::Cow<'l, str> {
        self.highlighter.highlight(line).into()
    }
}

impl validate::Validator for FayeHelper {
    fn validate(
        &self,
        ctx: &mut validate::ValidationContext,
    ) -> rustyline::Result<validate::ValidationResult> {
        let parser = Parser::new(ctx.input());
        match parser.parse() {
            Err(e) if matches!(e.kind, ParserErrorKind::UnclosedBracket) => {
                Ok(validate::ValidationResult::Incomplete)
            }
            _ => Ok(validate::ValidationResult::Valid(None)),
        }
    }
}
