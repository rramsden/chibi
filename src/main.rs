mod env;
mod types;

use types::*;
use env::Environment;
use std::io::{self, Write, BufRead};

fn main() {
    // global lisp environment
    let mut env = env::standard_env();

    loop {
        print!("> ");
        let expression = read();
        let result = interpret(parse(expression), &mut env);

        println!("{:?}", result);
    }
}

/// Take an input string and split on whitespace
///
/// Given (+ 1 1) it returns vec!['(', '+', '1', '1', ')']
fn tokenize(expression: String) -> Vec<String> {
    let expression = expression 
        .replace("(", " ( ")
        .replace(")", " ) ")
        .trim()
        .to_string();

    expression.split_whitespace().map(|s| s.to_string()).collect()
}

fn interpret(input: SyntaxTree, env: &mut Environment) -> Primitive {
    match input {
        SyntaxTree::List(tree) => interpret_list(tree, env),
        SyntaxTree::Element(primitive) => {
            match primitive {
                Primitive::Identifier(id) => {
                    match env.definitions.get(&id) {
                        Some(value) => return value.clone(),
                        _ => return Primitive::Identifier(id)
                    }
                }
                _ => primitive
            }
        }
    }
}

fn interpret_list(vec: Vec<SyntaxTree>, env: &mut Environment) -> Primitive {
    if vec.len() > 0 {
        let slice: Vec<Primitive> = vec.into_iter().map(|tree| interpret(tree, env)).collect();

        if let Primitive::Identifier(id) = slice.first().unwrap() {
            match env.stdlib.get(id) {
                Some(Primitive::Lambda(lambda)) => return lambda(slice[1..].to_vec(), env),
                _ => return Primitive::Tuple(slice)
            }
        } else {
            if slice.len() == 1 {
                slice[0].clone()
            } else {
                Primitive::Tuple(slice)
            }
        }
    } else {
        return Primitive::Null
    }
}

/// Builds an abstract syntax tree from tokenized input and returns an SyntaxTree
fn build_ast(mut input: Vec<String>, node: SyntaxTree) -> SyntaxTree {
    if input.len() == 0 {
        return node
    }

    let token = input.remove(0);
   
    if token == "(" {
        let new_node = SyntaxTree::List(Vec::new());

        if let SyntaxTree::List(mut list) = node {
            list.push(build_ast(input.clone(), new_node));
            return SyntaxTree::List(list);
        } else {
            return node;
        }
    } else if token == ")" {
        return node;
    } else {
        if let SyntaxTree::List(mut list) = node {
            list.push(categorize(token));
            return build_ast(input.clone(), SyntaxTree::List(list));
        } else {
            return node;
        }
    }
}

fn categorize(token: String) -> SyntaxTree {
    let first_ch = token.chars().next().unwrap();
    let last_ch = token.chars().last().unwrap();

    let value: Primitive;

    if token.parse::<f64>().is_ok() {
        value = if token.contains(".") {
            Primitive::Float(token.parse().unwrap())
        } else {
            Primitive::Integer(token.parse().unwrap())
        }
    } else if first_ch == '"' && last_ch == '"' {
        value = Primitive::String(token);
    } else {
        value = Primitive::Identifier(token);
    };

    return SyntaxTree::Element(value);
}

fn parse(expression: String) -> SyntaxTree {
    let tokens = tokenize(expression);
    let node = SyntaxTree::List(Vec::new());
    let ast = build_ast(tokens, node);
    println!("\n{:?}\n", ast);
    return ast;
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
