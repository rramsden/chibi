mod env;
mod types;
mod parser;
mod interpreter;

use std::fs;
use std::io::{self, Write, BufRead};
use parser::parse;
use interpreter::interpret;

fn main() {
    // global lisp environment
    let scope = env::standard_env();
    let stdlib = require("./stdlib.chibi");
    let (_, mut scope) = interpret(parse(&stdlib), scope, true);

    loop {
        print!("> ");
        let expression = read();
        let (result, result_scope) = interpret(parse(&expression), scope, true);
        scope = result_scope;
        println!("{:?}", result);
    }
}

fn require(path: &str) -> String {
    match fs::read_to_string(&path) {
        Ok(string) => string,
        _ => panic!("cannot read path")
    }
}

fn read() -> String {
    let _ = io::stdout().flush();
    let stdin = io::stdin();
    let line = stdin.lock()
        .lines()
        .next()
        .unwrap();

    match line {
        Ok(input) => input,
        Err(error) => panic!(error)
    }
}
