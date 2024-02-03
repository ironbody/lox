use std::{fs, io, process::exit};

use crate::{
    ast_printer::AstPrinter,
    parser::{Parser, ParserError},
    scanner::{Scanner, ScannerError},
    token::{Token, TokenType},
};

enum LoxError {
    Scanner(Vec<ScannerError>),
    Parser(ParserError),
}

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    pub fn run_file(&self, path: &str) {
        let content = fs::read_to_string(path).expect("Unable to read file");
        self.run(&content);

        if self.had_error {
            exit(65);
        }
    }

    pub fn run_promt(&mut self) {
        loop {
            print!("> ");
            let mut line = String::new();
            match io::stdin().read_line(&mut line) {
                Ok(x) if x > 0 => {
                    self.run(&line);
                    self.had_error = false;
                }
                _ => break,
            }
        }
    }

    fn run(&self, source: &str) -> Result<(), LoxError> {
        let tokens = Scanner::new(source.as_bytes())
            .scan_tokens()
            .map_err(|e| LoxError::Scanner(e))?;

        let mut parser = Parser::new(&tokens);
        let expression = parser.parse().map_err(|e| LoxError::Parser(e))?;

        println!("{}", AstPrinter.print(&expression));
        Ok(())
    }

    fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: usize, location: &str, message: &str) {
        println!("[line {line}] Error{location}: {message}");
        self.had_error = true;
    }

    fn error_token(&mut self, token: &Token, message: &str) {
        if token.ttype == TokenType::Eof {
            self.report(token.line, " at end", message)
        } else {
            self.report(token.line, &(" at '".to_string() + &token.lexeme), message)
        }
    }
}
