use crate::ast::{BinaryOp, Expr, Stmt, UnaryOp};
use crate::token::{Token, TokenKind};
use crate::value::Value;

use std::fmt;

#[derive(Debug)]
pub struct ParseError {
    message: String,
    line: usize,
    column: usize,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (строка {}, столбец {})",
            self.message,
            self.line,
            self.column
        )
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.statement()?);
        }

        Ok(statements)
    }

    fn is_at_end(&self) -> bool {
        matches!(&self.peek().kind, TokenKind::Eof)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> Token {
        let token = self.peek().clone();

        if !self.is_at_end() {
            self.current += 1;
        }

        token
    }

    fn check(&self, expected: &TokenKind) -> bool {
        std::mem::discriminant(&self.peek().kind)
            == std::mem::discriminant(expected)
    }

    fn consume(
        &mut self,
        expected: TokenKind,
        message: &str,
    ) -> Result<Token, ParseError> {
        if self.check(&expected) {
            return Ok(self.advance());
        }

        Err(self.error(message))
    }

    fn error(&self, message: &str) -> ParseError {
        ParseError {
            message: message.to_string(),
            line: self.peek().line,
            column: self.peek().column,
        }
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.check(&TokenKind::Let) {
            return self.let_declaration();
        }

        if self.check(&TokenKind::Print) {
            return self.print_statement();
        }

        if self.check(&TokenKind::If) {
            return self.if_statement();
        }

        if self.check(&TokenKind::While) {
            return self.while_statement();
        }

        if matches!(self.peek().kind, TokenKind::Identifier(_)) {
            return self.assignment_statement();
        }

        Err(self.error("Ожидалось объявление let, присваивание, print, if или while"))
    }

    fn let_declaration(&mut self) -> Result<Stmt, ParseError> {
        self.advance();

        let name = match self.peek().kind.clone() {
            TokenKind::Identifier(name) => {
                self.advance();
                name
            }

            _ => {
                return Err(self.error("Ожидалось имя переменной"));
            }
        };

        self.consume(
            TokenKind::Equal,
            "После имени переменной ожидался знак '='",
        )?;

        let initializer = self.expression()?;

        self.consume(
            TokenKind::Semicolon,
            "Ожидалась точка с запятой после объявления переменной",
        )?;

        Ok(Stmt::Let {
            name,
            initializer,
        })
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        self.advance();

        self.consume(
            TokenKind::LeftParen,
            "После print ожидалась открывающая скобка",
        )?;

        let expression = self.expression()?;

        self.consume(
            TokenKind::RightParen,
            "Ожидалась закрывающая скобка",
        )?;

        self.consume(
            TokenKind::Semicolon,
            "Ожидалась точка с запятой после print",
        )?;

        Ok(Stmt::Print { expression })
    }

    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // пропускаем 'if'

        let condition = self.expression()?;

        self.consume(
            TokenKind::LeftBrace,
            "Ожидалась открывающая фигурная скобка '{' после условия if",
        )?;

        let then_branch = self.block()?;

        let else_branch = if self.check(&TokenKind::Else) {
            self.advance(); // пропускаем 'else'

            if self.check(&TokenKind::If) {
                // Поддержка цепочки else if
                Some(vec![self.if_statement()?])
            } else {
                self.consume(
                    TokenKind::LeftBrace,
                    "Ожидалась открывающая фигурная скобка '{' после else",
                )?;
                Some(self.block()?)
            }
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn assignment_statement(&mut self) -> Result<Stmt, ParseError> {
        let name = match self.peek().kind.clone() {
            TokenKind::Identifier(name) => {
                self.advance();
                name
            }
            _ => {
                return Err(self.error("Ожидалось имя переменной"));
            }
        };

        self.consume(
            TokenKind::Equal,
            "После имени переменной ожидался знак '='",
        )?;

        let value = self.expression()?;

        self.consume(
            TokenKind::Semicolon,
            "Ожидалась точка с запятой после присваивания",
        )?;

        Ok(Stmt::Assign { name, value })
    }

    fn while_statement(&mut self) -> Result<Stmt, ParseError> {
        self.advance();

        let condition = self.expression()?;

        self.consume(
            TokenKind::LeftBrace,
            "Ожидалась открывающая фигурная скобка '{' после условия while",
        )?;

        let body = self.block()?;

        Ok(Stmt::While { condition, body })
    }

    fn block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = Vec::new();

        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            statements.push(self.statement()?);
        }

        self.consume(
            TokenKind::RightBrace,
            "Ожидалась закрывающая фигурная скобка '}' после блока",
        )?;

        Ok(statements)
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expression = self.comparison()?;

        loop {
            let operator = if self.check(&TokenKind::EqualEqual) {
                Some(BinaryOp::Equal)
            } else if self.check(&TokenKind::BangEqual) {
                Some(BinaryOp::NotEqual)
            } else {
                None
            };

            let Some(operator) = operator else {
                break;
            };

            self.advance();

            let right = self.comparison()?;

            expression = Expr::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expression)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expression = self.term()?;

        loop {
            let operator = if self.check(&TokenKind::Greater) {
                Some(BinaryOp::Greater)
            } else if self.check(&TokenKind::GreaterEqual) {
                Some(BinaryOp::GreaterEqual)
            } else if self.check(&TokenKind::Less) {
                Some(BinaryOp::Less)
            } else if self.check(&TokenKind::LessEqual) {
                Some(BinaryOp::LessEqual)
            } else {
                None
            };

            let Some(operator) = operator else {
                break;
            };

            self.advance();

            let right = self.term()?;

            expression = Expr::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expression)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expression = self.factor()?;

        loop {
            let operator = if self.check(&TokenKind::Plus) {
                Some(BinaryOp::Add)
            } else if self.check(&TokenKind::Minus) {
                Some(BinaryOp::Subtract)
            } else {
                None
            };

            let Some(operator) = operator else {
                break;
            };

            self.advance();

            let right = self.factor()?;

            expression = Expr::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expression)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expression = self.unary()?;

        loop {
            let operator = if self.check(&TokenKind::Star) {
                Some(BinaryOp::Multiply)
            } else if self.check(&TokenKind::Slash) {
                Some(BinaryOp::Divide)
            } else if self.check(&TokenKind::Percent) {
                Some(BinaryOp::Remainder)
            } else {
                None
            };

            let Some(operator) = operator else {
                break;
            };

            self.advance();

            let right = self.unary()?;

            expression = Expr::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expression)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.check(&TokenKind::Minus) {
            self.advance();

            let right = self.unary()?;

            return Ok(Expr::Unary {
                operator: UnaryOp::Negate,
                right: Box::new(right),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        let token = self.advance();

        match token.kind {
            TokenKind::Number(number) => {
                Ok(Expr::Literal(Value::Int(number)))
            }

            TokenKind::StringLiteral(text) => {
                Ok(Expr::Literal(Value::String(text)))
            }

            TokenKind::True => {
                Ok(Expr::Literal(Value::Bool(true)))
            }

            TokenKind::False => {
                Ok(Expr::Literal(Value::Bool(false)))
            }

            TokenKind::Identifier(name) => {
                Ok(Expr::Variable(name))
            }

            TokenKind::LeftParen => {
                let expression = self.expression()?;

                self.consume(
                    TokenKind::RightParen,
                    "Ожидалась закрывающая скобка",
                )?;

                Ok(expression)
            }

            _ => Err(self.error("Ожидалось выражение")),
        }
    }
}
