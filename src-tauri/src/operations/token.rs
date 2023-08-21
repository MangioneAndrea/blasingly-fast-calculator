use std::fmt::Debug;

use crate::generic_error::ParsingTokenError;

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    None,
    Integer(String),
    Float(String),
    Operation(char),
    ParenthesisOpen,
    ParenthesisClose,
}


impl Token {
    fn new(c: char) -> Token {
        match c {
            '0'..='9' => Self::Integer(c.to_string()),
            '+' | '*' | '/' | '-' => Self::Operation(c),
            '.' => Self::Float(String::from(".")),
            '(' => Self::ParenthesisOpen,
            ')' => Self::ParenthesisClose,
            _ => Self::None,
        }
    }

    pub fn get_grade(&self, parenthesis: usize) -> Option<usize> {
        match self {
            Token::Operation('*') => Some(1 * parenthesis * 1_000_000),
            Token::Operation('/') => Some(1 * parenthesis * 1_000_000),
            Token::Operation('+') => Some(0 * parenthesis * 1_000_000),
            Token::Operation('-') => Some(0 * parenthesis * 1_000_000),
            _ => None,
        }
    }

    pub fn can_be_followed_by(&self, other: &Token) -> bool {
        match (self, other) {
            (_, Self::None) => false,
            (Self::None, _) => false,
            (Self::Operation(_), Self::Operation(_)) => false,
            (Self::Operation(_), Self::ParenthesisClose) => false,
            (Self::Float(_), Self::ParenthesisOpen) => false,
            (Self::Integer(_), Self::ParenthesisOpen) => false,
            _ => true,
        }
    }

    pub fn in_none(&self) -> bool {
        matches!(*self, Self::None)
    }

    pub fn digest(self, c: char) -> Result<(Self, Option<Self>), ParsingTokenError> {
        let other = Token::new(c);

        if other.in_none() {
            return Err(ParsingTokenError::InvalidToken);
        }

        match (&self, &other) {
            (Self::None, _) => Ok((other, None)),
            (Self::Integer(n), Self::Integer(o)) => {
                Ok((Self::Integer(format!("{}{}", n, o).to_string()), None))
            }
            (Self::Float(n), Self::Integer(o)) => {
                Ok((Self::Float(format!("{}{}", n, o).to_string()), None))
            }

            (Self::Integer(n), Self::Float(f)) => {
                Ok((Self::Float(format!("{}{}", n, f).to_string()), None))
            }
            (Self::Float(_), Self::Float(_)) => Err(ParsingTokenError::TooManyDots),
            _ => Ok((self, Some(other))),
        }
    }
}
