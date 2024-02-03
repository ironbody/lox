use std::collections::HashMap;
use std::str::FromStr;
use thiserror::Error;

use crate::token::{Token, TokenType};


// Source for this error type:
// https://github.com/abesto/jlox-rs/blob/main/src/scanner.rs
#[derive(Error, Debug)]
pub enum ScannerError {
    #[error("Invalid UTF-8 character at {line}")]
    InvalidUtf8Char { line: usize },

    #[error("Unexpected character `{c}` at {line}")]
    UnexpectedCharacter { c: char, line: usize },

    #[error("Unterminated string starting at {line}")]
    UnterminatedString { line: usize },

    #[error("Unterminated /* block comment */ starting at {line}")]
    UnterminatedComment { line: usize },
}

pub struct Scanner<'a> {
    source: &'a [u8],
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<&'a str, TokenType>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
            keywords: HashMap::from([
                ("and", TokenType::And),
                ("class", TokenType::Class),
                ("else", TokenType::Else),
                ("false", TokenType::False),
                ("for", TokenType::For),
                ("fun", TokenType::Fun),
                ("if", TokenType::If),
                ("nil", TokenType::Nil),
                ("or", TokenType::Or),
                ("print", TokenType::Print),
                ("return", TokenType::Return),
                ("super", TokenType::Super),
                ("this", TokenType::This),
                ("true", TokenType::True),
                ("var", TokenType::Var),
                ("while", TokenType::While),
            ]),
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, Vec<ScannerError>> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut errors: Vec<ScannerError> = Vec::new();
        while !self.is_at_end() {
            self.start = self.current;
            let t = self.scan_token();
            // if let Ok(Some(tok)) = t {
            //     tokens.push(tok);
            // }
            match t {
                Ok(Some(tok)) => tokens.push(tok),
                Err(e) => errors.push(e),
                Ok(None) => (),
            }
        }
        tokens.push(Token::new(TokenType::Eof, "", self.line));
        if errors.len() != 0 {
            Err(errors)
        } else {
            Ok(tokens)
        }
    }

    fn scan_token(&mut self) -> Result<Option<Token>, ScannerError> {
        let c = self.advance();
        match c {
            b'(' => Ok(Some(self.make_token(TokenType::LeftParen))),
            b')' => Ok(Some(self.make_token(TokenType::RightParen))),
            b'{' => Ok(Some(self.make_token(TokenType::LeftBrace))),
            b'}' => Ok(Some(self.make_token(TokenType::RightBrace))),
            b',' => Ok(Some(self.make_token(TokenType::Comma))),
            b'.' => Ok(Some(self.make_token(TokenType::Dot))),
            b'-' => Ok(Some(self.make_token(TokenType::Minus))),
            b'+' => Ok(Some(self.make_token(TokenType::Plus))),
            b';' => Ok(Some(self.make_token(TokenType::Semicolon))),
            b'*' => Ok(Some(self.make_token(TokenType::Star))),
            b'!' => {
                if self.matches(b'=') {
                    Ok(Some(self.make_token(TokenType::BangEqual)))
                } else {
                    Ok(Some(self.make_token(TokenType::Bang)))
                }
            }
            b'=' => {
                if self.matches(b'=') {
                    Ok(Some(self.make_token(TokenType::EqualEqual)))
                } else {
                    Ok(Some(self.make_token(TokenType::Equal)))
                }
            }
            b'<' => {
                if self.matches(b'=') {
                    Ok(Some(self.make_token(TokenType::LessEqual)))
                } else {
                    Ok(Some(self.make_token(TokenType::Less)))
                }
            }
            b'>' => {
                if self.matches(b'=') {
                    Ok(Some(self.make_token(TokenType::GreaterEqual)))
                } else {
                    Ok(Some(self.make_token(TokenType::Greater)))
                }
            }
            b'/' => {
                if self.matches(b'/') {
                    while self.peek() != b'\n' && !self.is_at_end() {
                        self.advance();
                    }
                    Ok(None)
                } else {
                    Ok(Some(self.make_token(TokenType::Slash)))
                }
            }
            b' ' | b'\r' | b'\t' => Ok(None),
            b'\n' => {
                self.line += 1;
                Ok(None)
            }
            b'"' => self.string().map(Some),
            c if c.is_ascii_digit() => self.number().map(Some),
            c if self.is_alpha(c) => self.identifier().map(Some),
            c => Err(ScannerError::UnexpectedCharacter {
                c: c.into(),
                line: self.line,
            }),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> u8 {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn make_token(&self, ttype: TokenType) -> Token {
        let text = self
            .substring(self.start, self.current)
            .expect("Compiler error: tried to substring out of bounds");
        Token::new(ttype, &text, self.line)
    }

    fn matches(&mut self, c: u8) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.peek() != c {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() {
            return b'\0';
        }
        self.source[self.current]
    }

    fn peek_next(&self) -> u8 {
        if self.current + 1 >= self.source.len() {
            b'\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn string(&mut self) -> Result<Token, ScannerError> {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(ScannerError::UnterminatedString { line: self.line });
        }

        self.advance();

        let value = self.substring(self.start + 1, self.current - 1)?;
        Ok(Token::new(
            TokenType::String(value.clone()),
            &value,
            self.line,
        ))
    }

    fn substring(&self, start: usize, end: usize) -> Result<String, ScannerError> {
        String::from_utf8(self.source[start..end].to_vec())
            .map_err(|_| ScannerError::InvalidUtf8Char { line: self.line })
    }

    fn number(&mut self) -> Result<Token, ScannerError> {
        while self.peek().is_ascii_digit() {
            // consume digits
            self.advance();
        }

        // find decimal point
        if self.peek() == b'.' && self.peek_next().is_ascii_digit() {
            //consume the point
            self.advance();

            //consume the decimal part
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let value_string = self.substring(self.start, self.current)?;
        let value: f64 = f64::from_str(&value_string)
            .expect("Compiler error: Couldn't convert float string to value");
        Ok(Token::new(
            TokenType::Number(value),
            &value_string,
            self.line,
        ))
    }

    fn is_alpha(&self, character: u8) -> bool {
        character.is_ascii_alphabetic() || character == b'_'
    }

    fn is_alphanumeric(&self, character: u8) -> bool {
        self.is_alpha(character) || character.is_ascii_digit()
    }

    fn identifier(&mut self) -> Result<Token, ScannerError> {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }
        let text = self
            .substring(self.start, self.current)
            .expect("Compiler error: tried to substring out of bounds");

        match self.keywords.get(&text as &str) {
            Some(t) => Ok(self.make_token(t.clone())),
            None => Ok(self.make_token(TokenType::Identifier(text))),
        }
    }
}
