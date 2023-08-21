use crate::generic_error::ParsingTokenError;

pub(crate) mod token;
pub(crate) mod token_set;
pub(crate) mod token_tree;

#[derive(PartialEq, Clone, Debug)]
pub enum Executable {
    Sum,
    Sub,
    Mul,
    Div,
}

impl Executable {
    pub fn exec(&self, a: f32, b: f32) -> f32 {
        match self {
            Self::Sum => a + b,
            Self::Sub => a - b,
            Self::Mul => a * b,
            Self::Div => a / b,
        }
    }
}

impl TryFrom<char> for Executable {
    type Error = ParsingTokenError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Self::Sum),
            '-' => Ok(Self::Sub),
            '*' => Ok(Self::Mul),
            '/' => Ok(Self::Div),
            _ => Err(ParsingTokenError::OperationNotImplemented),
        }
    }
}

mod tests {
    use crate::operations::token::Token;

    #[test]
    fn test_invalid_floats() {
        let tokens = vec!["12.2.", "13.."];

        tokens
            .into_iter()
            .map(super::token_set::TokenSet::new)
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
            .map(super::token_set::TokenSet::new)
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
            .map(super::token_set::TokenSet::new)
            .for_each(|res| {
                assert!(res.is_ok());
                res.unwrap().0.into_iter().for_each(|token| {
                    assert!(matches!(token, super::token::Token::Float { .. }));
                })
            });
    }

    #[test]
    fn test_example_token() {
        let s = "7+12-3+1.1";

        let as_tokens = super::token_set::TokenSet::new(s);
        assert!(as_tokens.is_ok());

        let tokens = as_tokens.unwrap();
        assert_eq!(tokens.0.len(), 7);

        assert!(matches!(tokens.0[0], Token::Integer { .. }));
        assert!(matches!(tokens.0[1], Token::Operation { .. }));
        assert!(matches!(tokens.0[2], Token::Integer { .. }));
        assert!(matches!(tokens.0[3], Token::Operation { .. }));
        assert!(matches!(tokens.0[4], Token::Integer { .. }));
        assert!(matches!(tokens.0[5], Token::Operation { .. }));
        assert!(matches!(tokens.0[6], Token::Float { .. }));
    }

    #[test]
    fn test_evaluate_example_token_sum() {
        let s = "7+12-3+1.1";

        let as_tokens = super::token_set::TokenSet::new(s);
        assert!(as_tokens.is_ok());

        let tokens = as_tokens.unwrap();
        let tree = tokens.validate().unwrap().split();

        let solution = tree.solve();

        assert_eq!(solution, 17.1);
    }

    #[test]
    fn test_evaluate_example_token_sum_mul() {
        let s = "7+12*3+1+4*2";

        let as_tokens = super::token_set::TokenSet::new(s);
        assert!(as_tokens.is_ok());

        let tokens = as_tokens.unwrap();
        let tree = tokens.validate().unwrap().split();

        let solution = tree.solve();

        assert_eq!(solution, (7 + 12 * 3 + 1 + 4 * 2) as f32);
    }
    #[test]
    fn test_evaluate_example_token_sum_mul_parenthesis() {
        let s = "7+12*3+(1+4)*2";

        let as_tokens = super::token_set::TokenSet::new(s);
        assert!(as_tokens.is_ok());

        let tokens = as_tokens.unwrap();
        let tree = tokens.validate().unwrap().split();

        let solution = tree.solve();

        assert_eq!(solution, (7 + 12 * 3 + (1 + 4) * 2) as f32);
    }
}
