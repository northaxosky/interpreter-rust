use std::fmt;

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(LiteralValue),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self}")
    }
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    Bool(bool),
    Nil,
    Number(f64),
    Str(String),
}
impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LiteralValue::Bool(b) => write!(f, "{b}"),
            LiteralValue::Nil => write!(f, "nil"),
            LiteralValue::Number(n) => {
                todo!("{n}")
            }
            LiteralValue::Str(s) => write!(f, "{s}"),
        }
    }
}
