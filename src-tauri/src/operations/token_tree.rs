use super::token::Token;

pub enum TokenTree {
    Single(Token),
    Operation(Box<TokenTree>, Token, Box<TokenTree>),
    Parenthesis(Box<TokenTree>),
}

impl TokenTree {
    pub fn solve(&self) -> f32 {
        0.
    }
}
