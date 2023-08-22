use std::fmt::Debug;

use super::{binary_operations::BinaryOp, unary_operations::UnaryOp};
use crate::generic_error::ParsingTokenError;

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    None,
    Incomplete(String),
    Integer(String),
    Float(String),
    BinaryOperation(BinaryOp),
    UnaryOperation(UnaryOp),
    ParenthesisOpen,
    ParenthesisClose,
}

impl Token {
    fn new(c: char) -> Token {
        match c {
            '0'..='9' => Self::Integer(c.to_string()),
            '.' => Self::Float(String::from(".")),
            '(' => Self::ParenthesisOpen,
            ')' => Self::ParenthesisClose,
            _ => Self::Incomplete(c.to_string()), // Self::operation_from_str(c.to_string()).unwrap_or(Self::Incomplete(c.to_string())),
        }
    }

    fn operation_from_str(s: impl AsRef<str>) -> Option<Token> {
        let binary = BinaryOp::try_from(s.as_ref()).map(Self::BinaryOperation);
        let unary = UnaryOp::try_from(s.as_ref()).map(Self::UnaryOperation);

        binary.or(unary).ok()
    }

    // https://en.wikipedia.org/wiki/Order_of_operations
    pub fn get_grade(&self, parenthesis: usize) -> Option<usize> {
        match self {
            Token::BinaryOperation(BinaryOp::Sum) => Some(1 + parenthesis * 1_000_000),
            Token::BinaryOperation(BinaryOp::Sub) => Some(1 + parenthesis * 1_000_000),
            Token::BinaryOperation(BinaryOp::Mul) => Some(2 + parenthesis * 1_000_000),
            Token::BinaryOperation(BinaryOp::Div) => Some(2 + parenthesis * 1_000_000),
            Token::BinaryOperation(_) => Some(3 + parenthesis * 1_000_000),
            Token::UnaryOperation(_) => Some(4 + parenthesis * 1_000_000),
            _ => None,
        }
    }

    pub fn can_be_followed_by(&self, other: &Token) -> bool {
        match (self, other) {
            (_, Self::None) => false, // None cannot follow anything
            (Self::None, Self::BinaryOperation(_)) => false, // Cannot start with binary operation
            (Self::BinaryOperation(_), Self::BinaryOperation(_)) => false, // Binary operations cannot follow each other
            (Self::UnaryOperation(_), Self::BinaryOperation(_)) => false, // Binary operations cannot follow unary
            (Self::BinaryOperation(_), Self::ParenthesisClose) => false, // Binary need a number afterwards
            (Self::Float(_), Self::ParenthesisOpen) => false, // Float cannot be followed by parenthesis
            (Self::Integer(_), Self::ParenthesisOpen) => false, // Integer cannot be followed by parenthesis
            _ => true,
        }
    }

    pub fn in_none(&self) -> bool {
        matches!(*self, Self::None)
    }

    /// Add char to token. If the character cannot be digested, then the previous token is closed
    /// and a new one is also returned
    pub fn digest(self, c: char) -> Result<(Self, Option<Self>), ParsingTokenError> {
        let other = Token::new(c);

        if other.in_none() {
            return Err(ParsingTokenError::InvalidToken);
        }

        match (&self, &other) {
            (Self::Incomplete(a), Self::Incomplete(b)) => {
                Ok((Self::Incomplete(format!("{}{}", a, b)), None))
            }
            (Self::Incomplete(s), Self::Integer(o)) if s == &String::from("-") => {
                Ok((Self::Integer(format!("-{}", o)), None))
            }
            (Self::Incomplete(s), Self::Float(o)) if s == &String::from("-") => {
                Ok((Self::Float(format!("-{}", o)), None))
            }
            (Self::Integer(n), Self::Integer(o)) => {
                Ok((Self::Integer(format!("{}{}", n, o)), None))
            }
            (Self::Float(n), Self::Integer(o)) => Ok((Self::Float(format!("{}{}", n, o)), None)),

            (Self::Integer(n), Self::Float(f)) => Ok((Self::Float(format!("{}{}", n, f)), None)),
            (Self::Float(_), Self::Float(_)) => Err(ParsingTokenError::TooManyDots),
            (Self::Incomplete(s), _) => Token::operation_from_str(s)
                .map(|s| (s, Some(other)))
                .ok_or(ParsingTokenError::UnknownOperation),
            (Self::None, _) => Ok((other, None)),
            _ => Ok((self, Some(other))),
        }
    }
}
