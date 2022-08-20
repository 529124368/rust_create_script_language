mod ast;
mod exec;
mod parse;

use std::{env, fs::File, io::Read};

fn main() {
    let inputs: Vec<String> = env::args().collect();
    if inputs.len() >= 2 {
        let mut f = File::open(&inputs[1]).unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();
        let (_, b) = parse::to_ast(&buffer).unwrap();
        //println!("{:#?}", b);
        exec::do_exec(b);
    }
}
