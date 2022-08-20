use std::collections::HashMap;

use crate::ast;

#[derive(Clone)]
struct Zvals {
    type_name: String,
    float: f64,
    string: String,
}

fn match_expess(
    e: ast::Expression,
    global_params: &HashMap<String, Zvals>,
    local_params: &mut HashMap<String, Zvals>,
) -> Option<Zvals> {
    match e {
        ast::Expression::Main {
            opcode,
            left,
            right,
        } => None,
        ast::Expression::Zval {
            float,
            type_name,
            string,
        } => Some(Zvals {
            type_name,
            float,
            string,
        }),
        ast::Expression::Flg { name } => {
            if global_params.get(&name).is_some() {
                //全局
                let re = global_params.get(&name).unwrap().clone();
                return Some(re);
            } else if local_params.get(&name).is_some() {
                //局部
                let return_value = local_params.get(&name).unwrap().clone();
                local_params.remove(&name);
                return Some(return_value);
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
                Some(f) => {
                    if f.type_name == "number" {
                        println!("{}", f.float);
                    } else {
                        println!("{}", f.string);
                    }
                }
                None => print!(""),
            }
            None
        }
    }
}

pub fn do_exec(input: ast::Tree) {
    let mut global_params: HashMap<String, Zvals> = HashMap::new();
    let mut local_params: HashMap<String, Zvals> = HashMap::new();
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
                        ast::Expression::Zval {
                            float,
                            type_name,
                            string,
                        } => float,
                        _ => 0.0,
                    };
                    global_params.insert(
                        n,
                        Zvals {
                            type_name: "number".to_string(),
                            float: re,
                            string: "".to_string(),
                        },
                    );
                }
            }
        }
    }
}
