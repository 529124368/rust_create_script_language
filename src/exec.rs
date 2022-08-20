use std::collections::HashMap;

use crate::ast;

#[derive(Clone)]
struct Zvals {
    type_name: String,
    float: f64,
    string: String,
}

fn match_expess(
    e: ast::Token,
    global_params: &HashMap<String, Zvals>,
    local_params: &mut HashMap<String, Zvals>,
) -> Option<Zvals> {
    match e {
        ast::Token::Express {
            opcode,
            left,
            right,
        } => {
            let letf_num = match_expess(*left, global_params, local_params)
                .unwrap()
                .float;
            let right_num = match_expess(*right, global_params, local_params)
                .unwrap()
                .float;
            if opcode == ast::Opcode::Add {
                return Some(Zvals {
                    type_name: "number".to_string(),
                    float: letf_num + right_num,
                    string: "".to_string(),
                });
            }
            if opcode == ast::Opcode::Subtract {
                return Some(Zvals {
                    type_name: "number".to_string(),
                    float: letf_num - right_num,
                    string: "".to_string(),
                });
            }
            if opcode == ast::Opcode::Divide {
                return Some(Zvals {
                    type_name: "number".to_string(),
                    float: letf_num / right_num,
                    string: "".to_string(),
                });
            }
            if opcode == ast::Opcode::Multiply {
                return Some(Zvals {
                    type_name: "number".to_string(),
                    float: letf_num * right_num,
                    string: "".to_string(),
                });
            }
            None
        }
        ast::Token::Zval {
            float,
            type_name,
            string,
        } => Some(Zvals {
            type_name,
            float,
            string,
        }),
        ast::Token::Flg { name } => {
            if global_params.get(&name).is_some() {
                //全局
                let re = global_params.get(&name).unwrap().clone();
                return Some(re);
            } else if local_params.get(&name).is_some() {
                //局部
                let return_value = local_params.get(&name).unwrap().clone();
                return Some(return_value);
            } else {
                return None;
            }
        }
        ast::Token::Block { elements } => None,
        ast::Token::Assignment { name, token } => {
            let f = match_expess(*token, global_params, local_params).unwrap();
            local_params.insert(name, f);
            None
        }
        ast::Token::PrintLn { token } => {
            match match_expess(*token, global_params, local_params) {
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
                    ast::Token::Block { elements } => {
                        for i in elements {
                            match_expess(i, &global_params, &mut local_params);
                        }
                    }

                    _ => print!(""),
                },
                //全局变量
                ast::Program::GlobalParmDef { name: n, token: e } => {
                    let a = match_expess(e, &global_params, &mut local_params).unwrap();
                    global_params.insert(
                        n,
                        Zvals {
                            type_name: "number".to_string(),
                            float: a.float,
                            string: "".to_string(),
                        },
                    );
                }
            }
        }
    }
}
