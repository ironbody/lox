use crate::{
    expr::*, literal::Literal, token::{Token, TokenType}
};

#[derive(Debug)]
pub enum ParserError {
    Custom { message: String, token: Token },
}

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, ParserError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, ParserError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParserError> {
        let mut ex: Expr = self.comparison()?;

        use TokenType::*;
        while self.matches(&[BangEqual, EqualEqual]) {
            let op = self.previous();
            let right: Expr = self.comparison()?;
            let data = BinaryData {
                left: ex.into(),
                operator: op,
                right: right.into(),
            };
            ex = Expr::Binary(data);
        }

        return Ok(ex);
    }

    fn comparison(&mut self) -> Result<Expr, ParserError> {
        let mut ex = self.term()?;

        use TokenType::*;
        while self.matches(&[Greater, GreaterEqual, Less, LessEqual]) {
            let op = self.previous();
            let right: Expr = self.term()?;
            let data = BinaryData {
                left: ex.into(),
                operator: op,
                right: right.into(),
            };
            ex = Expr::Binary(data);
        }

        return Ok(ex);
    }

    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut ex = self.factor()?;

        use TokenType::*;
        while self.matches(&[Minus, Plus]) {
            let op = self.previous();
            let right: Expr = self.factor()?;
            let data = BinaryData {
                left: ex.into(),
                operator: op,
                right: right.into(),
            };
            ex = Expr::Binary(data);
        }

        return Ok(ex);
    }

    fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut ex = self.unary()?;

        use TokenType::*;
        while self.matches(&[Slash, Star]) {
            let op = self.previous();
            let right: Expr = self.unary()?;
            let data = BinaryData {
                left: ex.into(),
                operator: op,
                right: right.into(),
            };
            ex = Expr::Binary(data);
        }

        return Ok(ex);
    }

    fn unary(&mut self) -> Result<Expr, ParserError> {
        use TokenType::*;
        if self.matches(&[Bang, Minus]) {
            let op = self.previous();
            let right = self.unary()?;
            let data = UnaryData {
                operator: op,
                right: right.into(),
            };

            return Ok(Expr::Unary(data));
        }

        return self.primary();
    }

    fn primary(&mut self) -> Result<Expr, ParserError> {
        use TokenType::*;
        match self.peek().ttype {
            Number(n) => {
                self.advance();
                Ok(Expr::Literal(Literal::Number(n)))
            }
            String(s) => {
                self.advance();
                Ok(Expr::Literal(Literal::String(s)))
            }
            True => {
                self.advance();
                Ok(Expr::Literal(Literal::Bool(true)))
            }
            False => {
                self.advance();
                Ok(Expr::Literal(Literal::Bool(false)))
            }
            Nil => {
                self.advance();
                Ok(Expr::Literal(Literal::Nil))
            }
            LeftParen => {
                self.advance();
                let e = self.expression()?;
                let data = GroupingData {
                    expression: e.into(),
                };
                self.consume(RightParen, "Expect ')' after expression.")?;
                return Ok(Expr::Grouping(data));
            }
            _ => Err(ParserError::Custom {
                message: "Expect expression.".to_string(),
                token: self.peek(),
            }),
        }
    }

    fn matches(&mut self, values: &[TokenType]) -> bool {
        for t in values {
            if self.check(t) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn consume(&mut self, ttype: TokenType, message: &str) -> Result<Token, ParserError> {
        if self.check(&ttype) {
            return Ok(self.advance());
        }

        return Err(ParserError::Custom {
            message: message.into(),
            token: self.peek().clone(),
        });
    }

    fn check(&self, ttype: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        return self.peek().ttype == *ttype;
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        return self.peek().ttype == TokenType::Eof;
    }

    fn peek(&self) -> Token {
        return self.tokens[self.current].clone();
    }

    fn previous(&self) -> Token {
        return self.tokens[self.current - 1].clone();
    }

    fn synchronize(&mut self) {
        use TokenType::*;
        self.advance();
        while !self.is_at_end() {
            if self.previous().ttype == Semicolon {
                return;
            }

            match self.peek().ttype {
                Class | Fun | Var | For | If | While | Print | Return => return,
                _ => _ = self.advance(),
            }
        }
    }
}
