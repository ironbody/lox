mod ast_printer;
mod expr;
mod lox;
mod parser;
pub mod scanner;
pub mod token;
mod literal;
mod object;
mod interpreter;

use std::{env, fs, io, process::exit};

use ast_printer::AstPrinter;
use expr::{BinaryData, Expr, GroupingData, UnaryData};
use lox::Lox;
use token::{Token, TokenType};

use crate::{literal::Literal, scanner::Scanner};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut lox = Lox::new();
    if args.len() > 2 {
        println!("Usage: lox [script]");
        exit(64);
    } else if args.len() == 2 {
        lox.run_file(&args[1]);
    } else {
        lox.run_promt();
    }
}

fn test_ast_printer() {
    let udata = UnaryData {
        operator: Token {
            ttype: token::TokenType::Minus,
            lexeme: "-".to_string(),
            line: 1,
        },
        right: Expr::Literal(Literal::Number(123.0)).into(),
    };
    let unary = Expr::Unary(udata);

    let gdata = GroupingData {
        expression: Expr::Literal(Literal::Bool(false)).into(),
    };
    let group = Expr::Grouping(gdata);

    let bdata = BinaryData {
        left: unary.into(),
        operator: Token {
            ttype: token::TokenType::Star,
            lexeme: "*".to_string(),
            line: 1,
        },
        right: group.into(),
    };
    let binary = Expr::Binary(bdata);

    let output = AstPrinter.print(&binary);
    println!("{output}")
}
