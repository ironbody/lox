use crate::{expr::{Expr, ExpressionVisitor}, literal::Literal, token::{Token}};

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: &Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&self, name: &str, exprs: &[&Expr]) -> String{
        let mut output = String::new();
    
        output += "(";
        output += name;
    
        for e in exprs {
            output += " ";
            let expr_str = e.accept(self);
            output += &expr_str;
        }

        output += ")";
        return output;
    }
}

impl ExpressionVisitor<String> for AstPrinter {
    fn visit_binary(&self, left: &Expr, operand: &Token, right: &Expr) -> String{
        self.parenthesize(&operand.lexeme , &[left, right])
    }

    fn visit_unary(&self, operand: &Token, expr: &Expr) -> String {
        self.parenthesize(&operand.lexeme, &[expr])
    }

    fn visit_literal(&self, literal: &Literal) -> String {
        literal.to_string()
    }

    fn visit_grouping(&self, expr: &Expr) -> String {
        self.parenthesize("group", &[expr])
    }
    
}
