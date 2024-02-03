use crate::{expr::{Expr, ExpressionVisitor}, object::Object, token::TokenType};

pub enum InterpreterError{

}

pub struct Interpreter;

impl Interpreter {
    fn evaluate(&self, expr: &Expr) -> Result<Object, InterpreterError> {
        expr.accept(self)
    }
}

impl ExpressionVisitor<Result<Object, InterpreterError>> for Interpreter {
    fn visit_binary(&self, left: &crate::expr::Expr, operand: &crate::token::Token, right: &crate::expr::Expr) -> Result<Object, InterpreterError> {
        todo!()
    }

    fn visit_unary(&self, operand: &crate::token::Token, expr: &crate::expr::Expr) -> Result<Object, InterpreterError> {
        todo!()
        // let right = self.evaluate(expr)?;
        // use TokenType::*;
        // match operand.ttype {
        //     Minus => 
        // }
    }

    fn visit_literal(&self, literal: &crate::literal::Literal) -> Result<Object, InterpreterError> {
        Ok(Object::Literal(literal.clone()))
    }

    fn visit_grouping(&self, expr: &crate::expr::Expr) -> Result<Object, InterpreterError> {
        self.evaluate(expr)
    }
}