#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Ключевые слова
    Let,
    Print,
    If,
    Else,
    While,

    // Логические значения
    True,
    False,

    // Данные
    Number(i64),
    StringLiteral(String),
    Identifier(String),

    // Арифметические операторы
    Plus,
    Minus,
    Star,
    Slash,
    Percent,

    // Операторы сравнения
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Логические операторы
    AndAnd,
    OrOr,

    // Скобки
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    // Разделители
    Semicolon,

    // Конец файла
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}
