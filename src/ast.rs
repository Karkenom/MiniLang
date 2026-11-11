use crate::value::Value;

#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    // Арифметика
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,

    // Сравнение
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Negate,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Value),

    Variable(String),

    Binary {
        left: Box<Expr>,
        operator: BinaryOp,
        right: Box<Expr>,
    },

    Unary {
        operator: UnaryOp,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let {
        name: String,
        initializer: Expr,
    },

    Assign {
        name: String,
        value: Expr,
    },

    Print {
        expression: Expr,
    },

    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },

    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
}
