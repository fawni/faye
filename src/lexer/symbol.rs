/// A symbol used to identify a function or a variable
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct Symbol(pub String);

impl Symbol {
    /// Create a new symbol from a string
    pub fn from<T: Into<String>>(s: T) -> Self {
        Self(s.into())
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
