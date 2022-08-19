#[derive(Debug)]
pub enum Opcode {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub enum Expression {
    Main {
        opcode: Opcode,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Zval {
        float: f64,
    },
    Flg {
        name: String,
    },
    Block {
        elements: Vec<Expression>,
    },
    Assignment {
        name: String,
        expression: Box<Expression>,
    },
    PrintLn {
        expression: Box<Expression>,
    },
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub args: Vec<String>,
    pub content: Expression,
}

#[derive(Debug)]
pub enum Program {
    FunctionDef(Function),
    GlobalParmDef {
        name: String,
        expression: Expression,
    },
}

#[derive(Debug)]
pub struct Tree {
    pub root: Vec<Program>,
}

//方法
pub fn set_zval(value: f64) -> Expression {
    Expression::Zval { float: value }
}

pub fn set_flg(name: &str) -> Expression {
    Expression::Flg { name: name.into() }
}

pub fn assignment(name: &str, expression: Expression) -> Expression {
    Expression::Assignment {
        name: name.into(),
        expression: Box::new(expression),
    }
}
pub fn add(left: Expression, right: Expression) -> Expression {
    Expression::Main {
        opcode: Opcode::Add,
        left: Box::new(left),
        right: Box::new(right),
    }
}

pub fn define_function(name: &str, args: &[&str], content: Expression) -> Program {
    Program::FunctionDef(Function {
        name: name.to_string(),
        args: args.iter().map(|arg| arg.to_string()).collect(),
        content,
    })
}

pub fn difine_global_variable(name: &str, expression: Expression) -> Program {
    Program::GlobalParmDef {
        name: name.to_string(),
        expression,
    }
}
pub fn block(elements: Vec<Expression>) -> Expression {
    Expression::Block { elements }
}

pub fn ast_println(c: Expression) -> Expression {
    Expression::PrintLn {
        expression: Box::new(c),
    }
}
