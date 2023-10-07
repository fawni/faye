use faye::prelude::Highlighter;

pub struct FayeEditor {
    highlighter: Highlighter,
}

impl FayeEditor {
    pub const fn new(highlighter: Highlighter) -> Self {
        Self { highlighter }
    }
}

impl pomprt::Editor for FayeEditor {
    fn highlight(&self, buffer: &str) -> String {
        self.highlighter.highlight(buffer)
    }

    fn highlight_prompt(&self, prompt: &str, multiline: bool) -> String {
        if multiline {
            format!("\x1b[1;90m{prompt}\x1b[0m")
        } else {
            format!("\x1b[36m{prompt}\x1b[0m")
        }
    }
}
