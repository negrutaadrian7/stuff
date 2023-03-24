use std::fmt::Display;

#[derive(Clone, PartialEq, Debug)]
pub enum Constant {
    Identifier(String),
    String(String),
    Number(f32)
}

impl Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Constant::Identifier(s) => write!(f, "{}", s),
            Constant::String(s) => write!(f, "{:?}", s),
            Constant::Number(n) => write!(f, "{}", n),
        }
    }
}
