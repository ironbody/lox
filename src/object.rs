use crate::literal::Literal;

#[derive(Debug, Clone)]
pub enum Object{
    Literal(Literal)
}