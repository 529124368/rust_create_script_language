use std::collections::HashMap;

use crate::ast::{self};

pub fn do_exec(input: ast::Tree) {
    let mut params_box = HashMap::new();
    if !input.root.is_empty() {
        for i in input.root {
            match i {
                ast::Program::FunctionDef(fucntion) => {
                    // println!("{}", fucntion.name);
                    // println!("{:?}", fucntion.args);
                    // println!("{:?}", fucntion.content);
                    match fucntion.content {
                        ast::Expression::Block { elements } => {
                            for i in elements {
                                match i {
                                    ast::Expression::PrintLn { expression } => match *expression {
                                        ast::Expression::Zval { float } => {
                                            println!("{}", float)
                                        }
                                        ast::Expression::Flg { name } => {
                                            if params_box.get(&name).is_some() {
                                                println!("{}", params_box.get(&name).unwrap())
                                            }
                                        }
                                        _ => print!(""),
                                    },
                                    _ => print!(""),
                                }
                            }
                        }

                        _ => print!(""),
                    }
                }
                ast::Program::GlobalParmDef {
                    name: n,
                    expression: e,
                } => {
                    let re = match e {
                        ast::Expression::Zval { float } => float,
                        _ => 0.0,
                    };
                    params_box.insert(n, re);
                }
                _ => print!(""),
            }
        }
    }
}
