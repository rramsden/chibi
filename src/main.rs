mod env;
mod types;
mod parser;
mod interpreter;

use std::io::{self, Write, BufRead};
use parser::parse;
use interpreter::interpret;

fn main() {
    // global lisp environment
    let mut scope = env::standard_env();

    loop {
        print!("> ");
        let expression = read();
        let (result, result_scope) = interpret(parse(&expression), scope, true);
        scope = result_scope;
        println!("{:?}", result);
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
