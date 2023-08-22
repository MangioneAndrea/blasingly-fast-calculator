use super::token::Token;

#[derive(Debug)]
pub enum TokenTree {
    Single(Token),
    UnaryOperation(Token, Box<TokenTree>),
    BinaryOperation(Box<TokenTree>, Token, Box<TokenTree>),
    Parenthesis(Box<TokenTree>),
}

impl TokenTree {
    pub fn solve(&self) -> f32 {
        match self {
            Self::Single(Token::Float(n)) => n.parse().expect("float str was not float"),
            Self::Single(Token::Integer(n)) => n.parse().expect("int str was not flat"),
            Self::BinaryOperation(a, Token::BinaryOperation(op), b) => op.exec(a.solve(), b.solve()),
            Self::UnaryOperation(Token::UnaryOperation(op), b) => op.exec(b.solve()),
            Self::Parenthesis(content) => content.solve(),
            _ => 0.,
        }
    }
}
