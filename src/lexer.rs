pub struct Lexer<'a> {
    pub pos: usize,
    pub input: &'a str,
}

impl Lexer<'_> {
    pub fn new(input: &str) -> Lexer {
        Lexer { pos: 0, input }
    }

    pub fn current(&self) -> Option<char> {
        self.input.chars().nth(self.pos)
    }

    pub fn advance(&mut self) -> Option<char> {
        let c = self.current();
        self.pos += 1;
        c
    }

    pub fn is_symbol(c: char) -> bool {
        Symbol::try_from(c).is_ok()
    }

    pub fn read_next(&mut self) -> Result<Token, LexerError> {
        while self
            .current()
            .ok_or(LexerError::Other)?
            .is_ascii_whitespace()
        {
            self.advance();
        }

        match self.current().ok_or(LexerError::Other)? {
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
                let n = s.parse::<f64>()?;
                Ok(Token::Number(n))
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
                Err(LexerError::UnknownKeyword(s))
            }
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Result<Token, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.input.len() {
            None
        } else {
            Some(self.read_next())
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

#[derive(Debug, PartialEq)]
pub enum Symbol {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl TryFrom<char> for Symbol {
    type Error = LexerError;

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

#[derive(Debug, PartialEq)]
pub enum LexerError {
    InvalidSymbol(char),
    InvalidNumber(Option<String>),
    UnknownKeyword(String),
    Other,
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSymbol(c) => write!(f, "Invalid Symbol: '{c}' is not a valid symbol"),
            Self::InvalidNumber(n) => match n {
                Some(n) => write!(f, "Invalid Number: '{n}' is not a valid number"),
                None => write!(f, "Invalid Number"),
            },
            Self::UnknownKeyword(s) => write!(f, "Unknown Keyword: '{s}' is not a valid keyword"),
            Self::Other => write!(f, "ummmmmm... something went wrong idk"),
        }
    }
}

impl std::error::Error for LexerError {}

impl From<std::num::ParseFloatError> for LexerError {
    fn from(_: std::num::ParseFloatError) -> Self {
        Self::InvalidNumber(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new("(+ 14 25.5 333)");

        assert_eq!(lexer.next(), Some(Ok(Token::OpenParen)));
        assert_eq!(lexer.next(), Some(Ok(Token::Symbol(Symbol::Plus))));
        assert_eq!(lexer.next(), Some(Ok(Token::Number(14.0))));
        assert_eq!(lexer.next(), Some(Ok(Token::Number(25.5))));
        assert_eq!(lexer.next(), Some(Ok(Token::Number(333.0))));
        assert_eq!(lexer.next(), Some(Ok(Token::CloseParen)));
        assert_eq!(lexer.next(), None);
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
