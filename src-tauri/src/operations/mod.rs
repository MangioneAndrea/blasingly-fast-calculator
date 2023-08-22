use crate::generic_error::ParsingTokenError;

pub(crate) mod binary_operations;
pub(crate) mod token;
pub(crate) mod token_set;
pub(crate) mod token_tree;
pub(crate) mod unary_operations;

pub fn parse_string(input: &str) -> Result<f32, ParsingTokenError> {
    let set = token_set::TokenSet::new(input.replace(" ", "").as_str())?;
    let valid = set.validate()?;
    let tree = valid.split();

    Ok(tree.solve())
}

mod token_tests {
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
        use crate::operations::token::Token;

        let s = "7+12-3+1.1";

        let as_tokens = super::token_set::TokenSet::new(s);
        assert!(as_tokens.is_ok());

        let tokens = as_tokens.unwrap();
        assert_eq!(tokens.0.len(), 7);

        assert!(matches!(tokens.0[0], Token::Integer { .. }));
        assert!(matches!(tokens.0[1], Token::BinaryOperation { .. }));
        assert!(matches!(tokens.0[2], Token::Integer { .. }));
        assert!(matches!(tokens.0[3], Token::BinaryOperation { .. }));
        assert!(matches!(tokens.0[4], Token::Integer { .. }));
        assert!(matches!(tokens.0[5], Token::BinaryOperation { .. }));
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
