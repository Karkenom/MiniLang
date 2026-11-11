use crate::ast::{BinaryOp, Expr, Stmt, UnaryOp};
use crate::environment::Environment;
use crate::value::{calculate, print_value, Operation, Value};

const MAX_LOOP_ITERATIONS: usize = 10_000;

pub struct Evaluator {
    environment: Environment,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn execute(&mut self, statements: &[Stmt]) -> Result<(), String> {
        for statement in statements {
            self.execute_statement(statement)?;
        }

        Ok(())
    }

    fn execute_statement(&mut self, statement: &Stmt) -> Result<(), String> {
        match statement {
            Stmt::Let { name, initializer } => {
                let value = self.evaluate(initializer)?;

                self.environment.define(name.clone(), value);

                Ok(())
            }

            Stmt::Assign { name, value } => {
                let evaluated_value = self.evaluate(value)?;

                self.environment.assign(name, evaluated_value)?;

                Ok(())
            }

            Stmt::Print { expression } => {
                let value = self.evaluate(expression)?;

                print_value(&value);

                Ok(())
            }

            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond_value = self.evaluate(condition)?;

                match cond_value {
                    Value::Bool(true) => {
                        self.execute(then_branch)?;
                    }
                    Value::Bool(false) => {
                        if let Some(else_stmts) = else_branch {
                            self.execute(else_stmts)?;
                        }
                    }
                    _ => {
                        return Err(
                            "Условие в конструкции if должно иметь логический тип (true или false)".to_string(),
                        );
                    }
                }

                Ok(())
            }

            Stmt::While { condition, body } => {
                let mut iterations = 0;

                loop {
                    let cond_value = self.evaluate(condition)?;

                    match cond_value {
                        Value::Bool(true) => {
                            iterations += 1;
                            if iterations > MAX_LOOP_ITERATIONS {
                                return Err(format!(
                                    "Превышен лимит итераций цикла ({}) — возможен бесконечный цикл",
                                    MAX_LOOP_ITERATIONS
                                ));
                            }

                            self.execute(body)?;
                        }
                        Value::Bool(false) => {
                            break;
                        }
                        _ => {
                            return Err(
                                "Условие в цикле while должно иметь логический тип (true или false)".to_string(),
                            );
                        }
                    }
                }

                Ok(())
            }
        }
    }

    fn evaluate(&self, expression: &Expr) -> Result<Value, String> {
        match expression {
            Expr::Literal(value) => {
                Ok(value.clone())
            }

            Expr::Variable(name) => {
                self.environment.get(name)
            }

            Expr::Unary { operator, right } => {
                let value = self.evaluate(right)?;

                match operator {
                    UnaryOp::Negate => {
                        match value {
                            Value::Int(number) => {
                                let result = number
                                    .checked_neg()
                                    .ok_or_else(|| {
                                        "Переполнение при унарном минусе".to_string()
                                    })?;

                                Ok(Value::Int(result))
                            }

                            _ => Err(
                                "Унарный минус применим только к целым числам".to_string()
                            ),
                        }
                    }
                }
            }

            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_value = self.evaluate(left)?;
                let right_value = self.evaluate(right)?;

                self.evaluate_binary(
                    left_value,
                    *operator,
                    right_value,
                )
            }
        }
    }

    fn evaluate_binary(
        &self,
        left: Value,
        operator: BinaryOp,
        right: Value,
    ) -> Result<Value, String> {
        match (left, right) {
            (Value::Int(left), Value::Int(right)) => match operator {
                BinaryOp::Add => Ok(Value::Int(calculate(left, Operation::Add, right)?)),
                BinaryOp::Subtract => Ok(Value::Int(calculate(left, Operation::Subtract, right)?)),
                BinaryOp::Multiply => Ok(Value::Int(calculate(left, Operation::Multiply, right)?)),
                BinaryOp::Divide => Ok(Value::Int(calculate(left, Operation::Divide, right)?)),
                BinaryOp::Remainder => Ok(Value::Int(calculate(left, Operation::Remainder, right)?)),

                BinaryOp::Equal => Ok(Value::Bool(left == right)),
                BinaryOp::NotEqual => Ok(Value::Bool(left != right)),
                BinaryOp::Greater => Ok(Value::Bool(left > right)),
                BinaryOp::GreaterEqual => Ok(Value::Bool(left >= right)),
                BinaryOp::Less => Ok(Value::Bool(left < right)),
                BinaryOp::LessEqual => Ok(Value::Bool(left <= right)),
            },

            (Value::Bool(left), Value::Bool(right)) => match operator {
                BinaryOp::Equal => Ok(Value::Bool(left == right)),
                BinaryOp::NotEqual => Ok(Value::Bool(left != right)),
                _ => Err("Для логических значений доступны только операторы == и !=".to_string()),
            },

            (Value::String(left), Value::String(right)) => match operator {
                BinaryOp::Equal => Ok(Value::Bool(left == right)),
                BinaryOp::NotEqual => Ok(Value::Bool(left != right)),
                _ => Err("Для строк доступны только операторы == и !=".to_string()),
            },

            _ => Err(
                "Несовместимые типы операндов для бинарной операции".to_string()
            ),
        }
    }
}
