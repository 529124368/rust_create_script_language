use crate::ast;

pub fn do_exec(input: ast::Tree) {
    if input.root.len() > 0 {
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
                                    ast::Expression::PrintLn { expression } => {
                                        println!("{}", expression)
                                    }
                                    _ => print!(""),
                                }
                            }
                        }

                        _ => print!(""),
                    }
                }
                _ => print!(""),
            }
        }
    }
}
