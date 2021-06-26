mod parser;
mod syntax_tree;
mod lexer;
mod token;
mod keywords;

use parser::Parser;

use std::env;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(filename) = args.get(1) {
        let mut file = File::open(filename).expect("Unable to open the file");
        let mut source = String::new();

        file.read_to_string(&mut source).expect("Unable to read the file");

        let mut parser = Parser::new(&source);

        println!("{:?}", parser.parse());
    }
}
