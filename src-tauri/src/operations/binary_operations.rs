use crate::generic_error::ParsingTokenError;

#[derive(PartialEq, Clone, Debug)]
pub enum BinaryOp {
    Sum,
    Sub,
    Mul,
    Div,
}

impl BinaryOp {
    pub fn exec(&self, a: f32, b: f32) -> f32 {
        match self {
            Self::Sum => a + b,
            Self::Sub => a - b,
            Self::Mul => a * b,
            Self::Div => a / b,
        }
    }
}

impl TryFrom<char> for BinaryOp {
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
