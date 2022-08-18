use crate::ast;

pub fn do_exec(input: ast::Tree) {
    if input.root.len() > 0 {
        for i in input.root {
            match i {
                ast::Program::FunctionDef(fucntion) => {
                    println!("{}", fucntion.name);
                    println!("{:?}", fucntion.args);
                    println!("{:?}", fucntion.content);
                }
                _ => println!(""),
            }
        }
    }
}
