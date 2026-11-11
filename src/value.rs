#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Bool(bool),
    String(String),
    Void,
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
}

pub fn calculate(
    left: i64,
    operation: Operation,
    right: i64,
) -> Result<i64, String> {
    match operation {
        Operation::Add => {
            left.checked_add(right)
                .ok_or_else(|| "Переполнение при сложении".to_string())
        }

        Operation::Subtract => {
            left.checked_sub(right)
                .ok_or_else(|| "Переполнение при вычитании".to_string())
        }

        Operation::Multiply => {
            left.checked_mul(right)
                .ok_or_else(|| "Переполнение при умножении".to_string())
        }

        Operation::Divide => {
            if right == 0 {
                return Err("Деление на ноль".to_string());
            }

            left.checked_div(right)
                .ok_or_else(|| "Переполнение при делении".to_string())
        }

        Operation::Remainder => {
            if right == 0 {
                return Err("Деление на ноль".to_string());
            }

            left.checked_rem(right)
                .ok_or_else(|| "Ошибка при вычислении остатка".to_string())
        }
    }
}

pub fn print_value(value: &Value) {
    match value {
        Value::Int(number) => println!("{}", number),
        Value::Bool(boolean) => println!("{}", boolean),
        Value::String(text) => println!("{}", text),
        Value::Void => println!(""),
    }
}

pub fn show_calculation(
    left: i64,
    operation: Operation,
    right: i64,
) {
    match calculate(left, operation, right) {
        Ok(result) => println!("Результат: {}", result),
        Err(error) => println!("Ошибка: {}", error),
    }
}
