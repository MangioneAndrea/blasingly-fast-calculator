use std::{error::Error, fmt};

use crate::generic_error::MyError;

pub enum Token {
    None,
    Number(String),
    Operation(char),
    Dot,
    ParenthesisOpen,
    ParenthesisClose,
}

impl Token {
    fn new(c: char) -> Token {
        match c {
            '0'..='9' => Self::Number(c.to_string()),
            '+' | '*' | '/' | '-' => Self::Operation(c),
            '.' => Self::Dot,
            '(' => Self::ParenthesisOpen,
            ')' => Self::ParenthesisClose,
            _ => Self::None,
        }
    }

    fn in_none(&self) -> bool {
        matches!(*self, Self::None)
    }

    fn digest(self, c: char) -> Result<(Self, Option<Self>), Box<dyn Error>> {
        let other = Token::new(c);

        if other.in_none() {
            return Err(MyError::new(format!("Invalid token {}", c).as_str()));
        }

        match (&self, &other) {
            (Self::None, _) => Ok((other, None)),
            (Self::Number(n), Self::Number(o)) => {
                Ok((Self::Number(format!("{}{}", n, o).to_string()), None))
            }
            _ => Ok((self, Some(other))),
        }
    }
}

pub struct Tokenizable(Vec<Token>);

impl Tokenizable {
    pub fn new(s: &str) -> Result<Tokenizable, Box<dyn Error>> {
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
    fn test_invalid_tokens() {
    }
}
