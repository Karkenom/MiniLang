use crate::token::{Token, TokenKind};

use std::fmt;

#[derive(Debug)]
pub struct LexError {
    message: String,
    line: usize,
    column: usize,
}

impl fmt::Display for LexError {
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

pub struct Lexer {
    chars: Vec<char>,
    current: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            chars: source.chars().collect(),
            current: 0,
            line: 1,
            column: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.chars.len()
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.current).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.chars.get(self.current + 1).copied()
    }

    fn advance(&mut self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }

        let character = self.chars[self.current];

        self.current += 1;

        if character == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        Some(character)
    }

    fn get_lexeme(&self, start: usize) -> String {
        self.chars[start..self.current].iter().collect()
    }

    fn make_token(
        &self,
        kind: TokenKind,
        start: usize,
        line: usize,
        column: usize,
    ) -> Token {
        Token {
            kind,
            lexeme: self.get_lexeme(start),
            line,
            column,
        }
    }

    fn error(
        &self,
        message: &str,
        line: usize,
        column: usize,
    ) -> LexError {
        LexError {
            message: message.to_string(),
            line,
            column,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            let start = self.current;
            let line = self.line;
            let column = self.column;

            if let Some(token) = self.scan_token(start, line, column)? {
                tokens.push(token);
            }
        }

        tokens.push(Token {
            kind: TokenKind::Eof,
            lexeme: String::new(),
            line: self.line,
            column: self.column,
        });

        Ok(tokens)
    }

    fn scan_token(
        &mut self,
        start: usize,
        line: usize,
        column: usize,
    ) -> Result<Option<Token>, LexError> {
        let character = self.advance().unwrap();

        let token = match character {
            ' ' | '\r' | '\t' | '\n' => return Ok(None),

            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
            '/' => {
                if self.peek() == Some('/') {
                    while self.peek() != Some('\n') && !self.is_at_end() {
                        self.advance();
                    }

                    return Ok(None);
                }

                TokenKind::Slash
            }
            '%' => TokenKind::Percent,

            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,
            ';' => TokenKind::Semicolon,

            '=' => {
                if self.peek() == Some('=') {
                    self.advance();
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                }
            }

            '!' => {
                if self.peek() == Some('=') {
                    self.advance();
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                }
            }

            '>' => {
                if self.peek() == Some('=') {
                    self.advance();
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                }
            }

            '<' => {
                if self.peek() == Some('=') {
                    self.advance();
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                }
            }

            '&' => {
                if self.peek() == Some('&') {
                    self.advance();
                    TokenKind::AndAnd
                } else {
                    return Err(self.error(
                        "Ожидался второй символ '&'",
                        line,
                        column,
                    ));
                }
            }

            '|' => {
                if self.peek() == Some('|') {
                    self.advance();
                    TokenKind::OrOr
                } else {
                    return Err(self.error(
                        "Ожидался второй символ '|'",
                        line,
                        column,
                    ));
                }
            }

            '"' => {
                return self.scan_string(start, line, column);
            }

            '0'..='9' => {
                return self.scan_number(start, line, column);
            }

            'a'..='z' | 'A'..='Z' | '_' => {
                return self.scan_identifier(start, line, column);
            }

            _ => {
                return Err(self.error(
                    &format!("Неизвестный символ '{}'", character),
                    line,
                    column,
                ));
            }
        };

        Ok(Some(self.make_token(token, start, line, column)))
    }

    fn scan_number(
        &mut self,
        start: usize,
        line: usize,
        column: usize,
    ) -> Result<Option<Token>, LexError> {
        while matches!(self.peek(), Some('0'..='9')) {
            self.advance();
        }

        let lexeme = self.get_lexeme(start);

        let number = lexeme.parse::<i64>().map_err(|_| {
            self.error("Число выходит за пределы i64", line, column)
        })?;

        Ok(Some(self.make_token(
            TokenKind::Number(number),
            start,
            line,
            column,
        )))
    }

    fn scan_identifier(
        &mut self,
        start: usize,
        line: usize,
        column: usize,
    ) -> Result<Option<Token>, LexError> {
        while matches!(
            self.peek(),
            Some('a'..='z' | 'A'..='Z' | '0'..='9' | '_')
        ) {
            self.advance();
        }

        let text = self.get_lexeme(start);

        let kind = match text.as_str() {
            "let" => TokenKind::Let,
            "print" => TokenKind::Print,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            _ => TokenKind::Identifier(text),
        };

        Ok(Some(self.make_token(kind, start, line, column)))
    }

    fn scan_string(
        &mut self,
        start: usize,
        line: usize,
        column: usize,
    ) -> Result<Option<Token>, LexError> {
        let mut value = String::new();

        while let Some(character) = self.peek() {
            if character == '"' {
                self.advance();

                return Ok(Some(self.make_token(
                    TokenKind::StringLiteral(value),
                    start,
                    line,
                    column,
                )));
            }

            if character == '\\' {
                self.advance();

                let escaped = self.peek().ok_or_else(|| {
                    self.error("Незавершённая escape-последовательность", line, column)
                })?;

                self.advance();

                match escaped {
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    '"' => value.push('"'),
                    '\\' => value.push('\\'),
                    _ => {
                        return Err(self.error(
                            "Неизвестная escape-последовательность",
                            self.line,
                            self.column,
                        ));
                    }
                }

                continue;
            }

            value.push(character);
            self.advance();
        }

        Err(self.error("Незакрытая строка", line, column))
    }
}
