mod token;
mod lexer;
mod ast;
mod parser;
mod value;
mod environment;
mod evaluator;

use std::env;
use std::fs;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::evaluator::Evaluator;

fn read_source_file() -> Result<String, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(
            "Укажите файл: cargo run -- program.rpotop".to_string()
        );
    }

    let file_path = &args[1];

    if !file_path.ends_with(".rpotop") {
        return Err("Файл должен иметь расширение .rpotop".to_string());
    }

    fs::read_to_string(file_path)
        .map_err(|error| format!("Ошибка чтения файла: {}", error))
}

fn run() -> Result<(), String> {
    let source = read_source_file()?;

    // 1. Лексический анализ
    let mut lexer = Lexer::new(&source);

    let tokens = lexer
        .scan_tokens()
        .map_err(|error| format!("Ошибка Lexer: {}", error))?;

    // 2. Синтаксический анализ
    let mut parser = Parser::new(tokens);

    let statements = parser
        .parse()
        .map_err(|error| format!("Ошибка Parser: {}", error))?;

    // 3. Выполнение программы
    let mut evaluator = Evaluator::new();

    evaluator
        .execute(&statements)
        .map_err(|error| format!("Ошибка выполнения: {}", error))?;

    Ok(())
}

fn main() {
    println!("Чтения файла и начала програмы");

    if let Err(error) = run() {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}
