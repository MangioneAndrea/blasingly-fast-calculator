use std::{error::Error, fmt};

use crate::operations::token::Token;

#[derive(Debug, PartialEq)]
pub enum ParsingTokenError {
    TooManyDots,
    InvalidToken,
    ParenthesisClosedWithoutOpening,
    ParenthesisOpenedWithoutClosing,
    InvalidSequence,
    OperationNotImplemented,
    Empty,
    UnknownOperation,
}

impl Error for ParsingTokenError {
    fn description(&self) -> &str {
        &self.__description()
    }
}

impl ParsingTokenError {
    pub fn __description(&self) -> &str {
        match self {
            ParsingTokenError::TooManyDots => {
                "Too many dots in a float. A float can have only 1 dot"
            }
            ParsingTokenError::InvalidToken => "Invalid token given",
            ParsingTokenError::ParenthesisClosedWithoutOpening => {
                "Encountered ')' without respective opening"
            }
            ParsingTokenError::InvalidSequence => "Encountered invalid sequence",
            ParsingTokenError::ParenthesisOpenedWithoutClosing => {
                "Encountered '(' without respective closing"
            }
            ParsingTokenError::Empty => "",
            ParsingTokenError::OperationNotImplemented => "Operation is not implemented",
            ParsingTokenError::UnknownOperation => "Operation not known",
        }
    }
}

impl fmt::Display for ParsingTokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.__description().fmt(f)
    }
}
