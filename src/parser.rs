use crate::lexer::{Error as LexerError, Lexer, Symbol, Token};

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(f64),
    Symbol(Symbol),
    List(Vec<Expr>),
    CloseParen,
}

pub fn parse(lexer: &mut Lexer) -> Result<Expr, Error> {
    match lexer.next() {
        Some(Ok(Token::Number(n))) => Ok(Expr::Number(n)),
        Some(Ok(Token::Symbol(sym))) => Ok(Expr::Symbol(sym)),
        Some(Ok(Token::OpenParen)) => {
            let mut res: Vec<Expr> = Vec::new();
            loop {
                match parse(lexer) {
                    Ok(Expr::CloseParen) => break,
                    Ok(ex) => res.push(ex),
                    Err(e) => return Err(e),
                }
            }
            Ok(Expr::List(res))
        }
        Some(Ok(Token::CloseParen)) => Ok(Expr::CloseParen),
        Some(Err(e)) => Err(Error::LexerError(e)),
        None => Err(Error::Empty),
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    LexerError(LexerError),
    Empty,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "Nothing to parse"),
            Self::LexerError(e) => write!(f, "Lexer Error: {e}"),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_list() {
        let mut lexer = Lexer::new("(+ 1 2)");
        let res = parse(&mut lexer);
        assert_eq!(
            res,
            Ok(Expr::List(vec![
                Expr::Symbol(Symbol::Plus),
                Expr::Number(1.0),
                Expr::Number(2.0),
            ]))
        );
    }

    #[test]
    fn parse_nested_list() {
        let mut lexer = Lexer::new("(+ 2.5 64 (* 2 3))");
        let res = parse(&mut lexer);
        assert_eq!(
            res,
            Ok(Expr::List(vec![
                Expr::Symbol(Symbol::Plus),
                Expr::Number(2.5),
                Expr::Number(64.0),
                Expr::List(vec![
                    Expr::Symbol(Symbol::Multiply),
                    Expr::Number(2.0),
                    Expr::Number(3.0),
                ]),
            ]))
        );
    }
}
