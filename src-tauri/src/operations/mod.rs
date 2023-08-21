use std::fmt::Debug;

use crate::generic_error::ParsingTokenError;

pub enum Token {
    None,
    Integer(String),
    Float(String),
    Operation(char),
    ParenthesisOpen,
    ParenthesisClose,
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::None => "None".to_string(),
                Self::Integer(i) => format!("Integer({})", i),
                Self::Float(i) => format!("Float({})", i),
                Self::Operation(i) => format!("Operation({})", i),
                Self::ParenthesisClose => "ParenthesisClose".to_string(),
                Self::ParenthesisOpen => "ParenthesisOpen".to_string(),
            }
        )
    }
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

    fn in_none(&self) -> bool {
        matches!(*self, Self::None)
    }

    fn digest(self, c: char) -> Result<(Self, Option<Self>), ParsingTokenError> {
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

pub struct Tokenizable(Vec<Token>);

impl Tokenizable {
    pub fn new(s: &str) -> Result<Tokenizable, ParsingTokenError> {
        let mut result = vec![];
        let mut current_token = Token::None;
        for c in s.chars() {
            current_token = match current_token.digest(c) {
                // If a token failed, the whole thing fails
                Err(e) => return Err(e),
                // If it yielded 2 tokens, the previous one can be "committed"
                Ok((old, Some(new))) => {
                    result.push(old);
                    new
                }
                // The old token was edited
                Ok((old, None)) => old,
            };
        }

        if !current_token.in_none() {
            result.push(current_token);
        }

        Ok(Tokenizable(result))
    }
}

mod tests {

    #[test]
    fn test_invalid_floats() {
        let tokens = vec!["12.2.", "13.."];

        tokens
            .into_iter()
            .map(super::Tokenizable::new)
            .for_each(|res| {
                let err = res.err().unwrap();
                assert_eq!(err, crate::generic_error::ParsingTokenError::TooManyDots);
            });
    }
    #[test]
    fn test_invalid_tokens() {
        let tokens = vec!["12$12", "a", "12b", "&"];

        tokens
            .into_iter()
            .map(super::Tokenizable::new)
            .for_each(|res| {
                let err = res.err().unwrap();
                assert_eq!(err, crate::generic_error::ParsingTokenError::InvalidToken);
            });
    }

    #[test]
    fn test_valid_floats() {
        let tokens = vec!["12.12", ".12", "12.", "0.0"];

        tokens
            .into_iter()
            .map(super::Tokenizable::new)
            .for_each(|res| {
                assert!(res.is_ok());
                res.unwrap().0.into_iter().for_each(|token| {
                    assert!(format!("{:?}", token).contains("Float"));
                })
            });
    }
}
