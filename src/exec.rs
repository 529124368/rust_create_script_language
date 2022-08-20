use std::{collections::HashMap, ops::Deref};

use crate::ast;

fn match_expess(
    e: ast::Expression,
    global_params: &HashMap<String, f64>,
    local_params: &mut HashMap<String, f64>,
) -> Option<f64> {
    match e {
        ast::Expression::Main {
            opcode,
            left,
            right,
        } => None,
        ast::Expression::Zval { float } => Some(float),
        ast::Expression::Flg { name } => {
            if global_params.get(&name).is_some() {
                //全局
                return Some(*global_params.get(&name).unwrap());
            } else if local_params.get(&name).is_some() {
                //局部
                let return_Value = *local_params.get(&name).unwrap();
                local_params.remove(&name);
                return Some(return_Value);
            } else {
                return None;
            }
        }
        ast::Expression::Block { elements } => None,
        ast::Expression::Assignment { name, expression } => {
            let f = match_expess(*expression, &global_params, local_params).unwrap();
            local_params.insert(name, f);
            None
        }
        ast::Expression::PrintLn { expression } => {
            match match_expess(*expression, &global_params, local_params) {
                Some(f) => println!("{}", f),
                None => print!(""),
            }
            None
        }
    }
}

pub fn do_exec(input: ast::Tree) {
    let mut global_params = HashMap::new();
    let mut local_params = HashMap::new();
    if !input.root.is_empty() {
        for i in input.root {
            match i {
                //方法
                ast::Program::FunctionDef(fucntion) => match fucntion.content {
                    ast::Expression::Block { elements } => {
                        for i in elements {
                            match_expess(i, &global_params, &mut local_params);
                        }
                    }

                    _ => print!(""),
                },
                //全局变量
                ast::Program::GlobalParmDef {
                    name: n,
                    expression: e,
                } => {
                    let re = match e {
                        ast::Expression::Zval { float } => float,
                        _ => 0.0,
                    };
                    global_params.insert(n, re);
                }
                _ => print!(""),
            }
        }
    }
}
