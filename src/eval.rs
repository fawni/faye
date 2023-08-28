use crate::{
    lexer::Symbol,
    parser::{Expr, Node},
};

pub fn eval(ast: Node) -> Result<f64, Error> {
    match ast {
        Node(Expr::Number(n), _) => Ok(n),
        Node(Expr::List(list), location) => {
            if list.len() < 2 {
                return Err(Error::new(ErrorKind::MissingArguments, location));
            }

            let Expr::Symbol(func) = list[0].clone() else {
                return Err(Error::new(ErrorKind::InvalidFunction, location));
            };

            let args = list[1..]
                .iter()
                .map(|expr| match eval(Node(expr.clone(), location)) {
                    Ok(n) => Ok(n),
                    Err(err) => Err(err),
                })
                .collect::<Result<Vec<f64>, Error>>()?;

            match func {
                Symbol::Plus => Ok(args.into_iter().sum()),
                Symbol::Minus => Ok(args
                    .into_iter()
                    .reduce(|acc, x| acc - x)
                    .ok_or(Error::new(ErrorKind::CalculationError, location))?),
                Symbol::Multiply => Ok(args
                    .into_iter()
                    .reduce(|acc, x| acc * x)
                    .ok_or(Error::new(ErrorKind::CalculationError, location))?),
                Symbol::Divide => Ok(args
                    .into_iter()
                    .reduce(|acc, x| acc / x)
                    .ok_or(Error::new(ErrorKind::CalculationError, location))?),
            }
        }
        Node(Expr::Symbol(sym), location) => {
            Err(Error::new(ErrorKind::SymbolMisplaced(sym), location))
        }
        Node(Expr::CloseParen, location) => Err(Error::new(ErrorKind::Unreachable, location)),
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
    Unreachable,
    SymbolMisplaced(Symbol),
    MissingArguments,
    InvalidFunction,
    CalculationError,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unreachable => write!(f, "Supposedly unreachable code reached"),
            Self::SymbolMisplaced(sym) => write!(f, "Symbol '{sym}' should not be here"),
            Self::MissingArguments => write!(f, "Expression is missing arguments"),
            Self::InvalidFunction => write!(f, "Function undefined"),
            Self::CalculationError => write!(f, "Calculation error"),
        }
    }
}
