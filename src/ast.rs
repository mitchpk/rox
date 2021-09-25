use crate::token::Token;
use std::fmt;

pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Token),
    Unary(Token, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Expr::Binary(left, operator, right) => {
                write!(f, "({} {} {})", operator.lexeme, left, right)
            }

            Expr::Grouping(expression) => write!(f, "(group {})", expression),

            Expr::Literal(token) => write!(f, "{:?}", token.kind),

            Expr::Unary(operator, right) => write!(f, "({} {})", operator.lexeme, right),
        }
    }
}
