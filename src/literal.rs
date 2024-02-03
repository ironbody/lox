use std::fmt;

#[derive(Debug, Clone)]
pub enum Literal{
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(num) => write!(f, "{num}"),
            Self::String(s) => write!(f, "{s}"),
            Self::Nil => write!(f, "nil"),
            Self::Bool(true) => write!(f, "true"),
            Self::Bool(false) => write!(f, "false"),
        }
    }
} 