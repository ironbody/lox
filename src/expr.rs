use core::fmt;
use std::rc::Rc;

use crate::{literal::Literal, token::Token};


pub trait ExpressionVisitor<T> {
    fn visit_binary(&self, left: &Expr, operand: &Token, right: &Expr) -> T;
    fn visit_unary(&self, operand: &Token, expr: &Expr) -> T;
    fn visit_literal(&self, literal: &Literal) -> T;
    fn visit_grouping(&self, expr: &Expr) -> T;
}

pub enum Expr {
    Binary(BinaryData),
    Unary(UnaryData),
    Literal(Literal),
    Grouping(GroupingData)

}

impl Expr {
    pub fn accept<T, V: ExpressionVisitor<T>>(&self, visitor: &V) -> T{
        match self {
            Expr::Binary(data) => visitor.visit_binary(&data.left, &data.operator, &data.right),
            Expr::Unary(data) => visitor.visit_unary(&data.operator, &data.right),
            Expr::Literal(data) => visitor.visit_literal(data),
            Expr::Grouping(data) => visitor.visit_grouping(&data.expression),
        }
    }
}


pub struct BinaryData{
    pub left: Rc<Expr>,
    pub right: Rc<Expr>,
    pub operator: Token,
}

pub struct UnaryData{
    pub right: Rc<Expr>,
    pub operator: Token,
}

pub struct GroupingData{
    pub expression: Rc<Expr>,
}



