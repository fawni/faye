pub struct Lexer<'a> {
    pos: usize,
    line: usize,
    input: &'a str,
}

impl Lexer<'_> {
    pub const fn new(input: &str) -> Lexer {
        Lexer {
            pos: 0,
            line: 0,
            input,
        }
    }

    pub const fn location(&self, len: usize) -> (usize, usize) {
        (self.line, self.pos - len)
    }

    fn current(&self) -> Option<char> {
        self.input.chars().nth(self.pos)
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.current();
        self.pos += 1;
        c
    }

    fn is_symbol(c: char) -> bool {
        Symbol::try_from(c).is_ok()
    }

    fn read(&mut self) -> Result<Token, Error> {
        while self
            .current()
            .ok_or_else(|| Error::new(ErrorKind::Other, self.location(0)))?
            .is_ascii_whitespace()
        {
            self.advance();
        }

        match self
            .current()
            .ok_or_else(|| Error::new(ErrorKind::Other, self.location(0)))?
        {
            '(' => {
                self.advance();
                Ok(Token::OpenParen)
            }
            ')' => {
                self.advance();
                Ok(Token::CloseParen)
            }
            '0'..='9' => {
                let mut s = String::new();
                while let Some(c) = self.current() {
                    if !c.is_ascii_digit() && c != '.' {
                        break;
                    }

                    s.push(c);
                    self.advance();
                }
                let len = s.len();
                let n = s
                    .parse::<f64>()
                    .ok()
                    .ok_or_else(|| Error::new(ErrorKind::InvalidNumber(s), self.location(len)))?;
                Ok(Token::Number(n))
            }
            '\n' => {
                self.advance();
                self.line += 1;
                self.read()
            }
            sym if Self::is_symbol(sym) => {
                self.advance();
                Ok(Token::Symbol(Symbol::try_from(sym).unwrap()))
            }
            _ => {
                let mut s = String::new();
                while let Some(c) = self.current() {
                    if !c.is_ascii_alphanumeric() {
                        break;
                    }

                    s.push(c);
                    self.advance();
                }
                let len = s.len();
                Err(Error::new(ErrorKind::UnknownKeyword(s), self.location(len)))
            }
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.input.len() {
            None
        } else {
            Some(self.read())
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    OpenParen,
    CloseParen,
    Symbol(Symbol),
    Number(f64),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Symbol {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl Symbol {
    pub const fn len(&self) -> usize {
        1
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Multiply => write!(f, "*"),
            Self::Divide => write!(f, "/"),
        }
    }
}

impl TryFrom<char> for Symbol {
    type Error = ErrorKind;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '+' => Ok(Self::Plus),
            '-' => Ok(Self::Minus),
            '*' => Ok(Self::Multiply),
            '/' => Ok(Self::Divide),
            _ => Err(Self::Error::InvalidSymbol(c)),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Error {
    pub kind: ErrorKind,
    pub location: (usize, usize),
}

impl Error {
    pub const fn new(kind: ErrorKind, location: (usize, usize)) -> Self {
        Self { kind, location }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (line, col) = self.location;
        write!(f, "{}:{} {}", line, col, self.kind)
    }
}

impl std::error::Error for Error {}

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorKind {
    InvalidSymbol(char),
    InvalidNumber(String),
    UnknownKeyword(String),
    Other,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSymbol(c) => write!(f, "'{c}' is not a valid symbol"),
            Self::InvalidNumber(s) => write!(f, "Could not parse number '{s}'"),
            Self::UnknownKeyword(s) => write!(f, "Unknown Keyword: '{s}' is not a valid keyword"),
            Self::Other => write!(f, "ummmmmm... something went wrong idk"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer() {
        let mut lexer = Lexer::new("(+ 14 25.5 333 (* 2 5))");

        assert_eq!(lexer.next(), Some(Ok(Token::OpenParen)));
        assert_eq!(lexer.next(), Some(Ok(Token::Symbol(Symbol::Plus))));
        assert_eq!(lexer.next(), Some(Ok(Token::Number(14.0))));
        assert_eq!(lexer.next(), Some(Ok(Token::Number(25.5))));
        assert_eq!(lexer.next(), Some(Ok(Token::Number(333.0))));
        assert_eq!(lexer.next(), Some(Ok(Token::OpenParen)));
        assert_eq!(lexer.next(), Some(Ok(Token::Symbol(Symbol::Multiply))));
        assert_eq!(lexer.next(), Some(Ok(Token::Number(2.0))));
        assert_eq!(lexer.next(), Some(Ok(Token::Number(5.0))));
        assert_eq!(lexer.next(), Some(Ok(Token::CloseParen)));
        assert_eq!(lexer.next(), Some(Ok(Token::CloseParen)));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn parse_numbers() {
        let mut lexer = Lexer::new("2 55 3.144 0.0001 1.1.1");

        assert_eq!(lexer.next(), Some(Ok(Token::Number(2.0))));
        assert_eq!(lexer.next(), Some(Ok(Token::Number(55.0))));
        assert_eq!(lexer.next(), Some(Ok(Token::Number(3.144))));
        assert_eq!(lexer.next(), Some(Ok(Token::Number(0.0001))));
        assert_eq!(
            lexer.next(),
            Some(Err(Error::new(
                ErrorKind::InvalidNumber("1.1.1".to_owned()),
                (0, 18),
            )))
        );
    }

    #[test]
    fn is_symbol() {
        assert!(Lexer::is_symbol('+'));
        assert!(Lexer::is_symbol('-'));
        assert!(Lexer::is_symbol('*'));
        assert!(Lexer::is_symbol('/'));
        assert!(!Lexer::is_symbol('a'));
    }
}
