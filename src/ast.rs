#[derive(Debug, Clone, PartialEq)]
pub enum Opcode {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    Number,
    String,
}

#[derive(Debug, Clone)]
pub enum Token {
    Express {
        opcode: Opcode,
        left: Box<Token>,
        right: Box<Token>,
    },
    Zval {
        type_name: ValueType,
        float: f64,
        string: String,
    },
    Flg {
        name: String,
    },
    Block {
        elements: Vec<Token>,
    },
    Assignment {
        name: String,
        token: Box<Token>,
    },
    PrintLn {
        token: Box<Token>,
    },
    CallFunction {
        name: String,
        args: Vec<String>,
    },
}

#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub args: Vec<String>,
    pub content: Token,
}

#[derive(Clone, Debug)]
pub enum Program {
    FunctionDef(Function),
    GlobalParmDef { name: String, token: Token },
}

#[derive(Clone, Debug)]
pub struct Tree {
    pub root: Vec<Program>,
}

//方法
pub fn set_zval(value: f64, vstr: &str, type_name: ValueType) -> Token {
    Token::Zval {
        float: value,
        type_name,
        string: vstr.to_string(),
    }
}

pub fn set_flg(name: &str) -> Token {
    Token::Flg { name: name.into() }
}

pub fn assignment(name: &str, token: Token) -> Token {
    Token::Assignment {
        name: name.into(),
        token: Box::new(token),
    }
}
pub fn add(left: Token, right: Token) -> Token {
    Token::Express {
        opcode: Opcode::Add,
        left: Box::new(left),
        right: Box::new(right),
    }
}

pub fn subtract(left: Token, right: Token) -> Token {
    Token::Express {
        opcode: Opcode::Subtract,
        left: Box::new(left),
        right: Box::new(right),
    }
}

pub fn multiply(left: Token, right: Token) -> Token {
    Token::Express {
        opcode: Opcode::Multiply,
        left: Box::new(left),
        right: Box::new(right),
    }
}

pub fn divide(left: Token, right: Token) -> Token {
    Token::Express {
        opcode: Opcode::Divide,
        left: Box::new(left),
        right: Box::new(right),
    }
}

pub fn define_function(name: &str, args: &[&str], content: Token) -> Program {
    Program::FunctionDef(Function {
        name: name.to_string(),
        args: args.iter().map(|arg| arg.to_string()).collect(),
        content,
    })
}

pub fn define_call_function(name: &str, args: &[&str]) -> Token {
    Token::CallFunction {
        name: name.to_string(),
        args: args.iter().map(|arg| arg.to_string()).collect(),
    }
}

pub fn define_global_variable(name: &str, token: Token) -> Program {
    Program::GlobalParmDef {
        name: name.to_string(),
        token,
    }
}
pub fn block(elements: Vec<Token>) -> Token {
    Token::Block { elements }
}

pub fn ast_println(c: Token) -> Token {
    Token::PrintLn { token: Box::new(c) }
}
