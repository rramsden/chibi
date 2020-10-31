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
        SyntaxTree::List(list) => interpret_list(list, env),
        SyntaxTree::Element(primitive) => {
            match primitive {
                Primitive::Identifier(id) => {
                    match env.variables.get(&id) {
                        Some(SyntaxTree::Element(primitive)) => return primitive.clone(),
                        _ => return Primitive::Identifier(id)
                    }
                }
                _ => primitive
            }
        }
    }
}

fn interpret_list(mut vec: Vec<SyntaxTree>, env: &mut Environment) -> Primitive { // Defining functions
    if vec.len() == 3 {
        let define = vec[0].clone();
        let label = vec[1].clone();
        let body = vec[2].clone();

        // Define statements can have two bodies
        //
        // 1. (define x body)
        // 2. (define (sum-of-squares x y) body)
        //
        match define {
            SyntaxTree::Element(Primitive::Identifier(ref v)) if v == "define" => {
                match label {
                    SyntaxTree::List(vec) => {
                        let slice = &vec[..];
                        let label = &slice[0];
                        let arguments: Vec<SyntaxTree> = slice[1..].into();

                        if let SyntaxTree::Element(Primitive::Identifier(id)) = label {
                            env.functions.insert(id.clone(), (arguments, body));
                            return Primitive::Identifier(id.clone());
                        } else {
                            panic!("expected unknown type");
                        }
                    },
                    SyntaxTree::Element(Primitive::Identifier(id)) => {
                        if let SyntaxTree::Element(primitive) = body {
                            env.variables.insert(id.clone(), SyntaxTree::Element(primitive));
                            return Primitive::Identifier(id);
                        } else {
                            panic!("don't know how to handle define statement")
                        }
                    },
                    _ => panic!("don't know how to handle define statement")
                }

            },
            _ => ()
        }
    }

    if vec.len() > 0 {
        let first = &vec[0];

        // check left-most value
        if let SyntaxTree::Element(Primitive::Identifier(id)) = first {
            match env.functions.get(id) {
                Some((signature, body)) => {
                    let mut params: Vec<SyntaxTree> = vec![];
                    for (i, _) in vec[1..].into_iter().enumerate() {
                        let new_vec = vec[i + 1].clone();
                        params.push(new_vec);
                    }

                    vec = vec![SyntaxTree::Element(apply(signature.to_vec(), params, body.clone(), env))];
                },
                _ => {}
            }
        }

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

// signature: [Identifier(x)]
// params: [Integer(2)]
// body: SyntaxTree
fn apply(signature: Vec<SyntaxTree>, params: Vec<SyntaxTree>, body: SyntaxTree, env: &mut Environment) -> Primitive {
    for (i, id) in signature.into_iter().enumerate() {
        if let SyntaxTree::Element(Primitive::Identifier(label)) = id {
            env.variables.insert(label, params[i].clone());
        }
    }

    return interpret(body, env);
}

/// Builds an abstract syntax tree from tokenized input and returns a SyntaxTree
fn parenthesize(input: &mut Vec<String>, node: SyntaxTree) -> SyntaxTree {
    if input.len() == 0 {
        return node
    }

    let token = input.remove(0);
   
    if token == "(" {
        let new_node = SyntaxTree::List(Vec::new());

        if let SyntaxTree::List(mut list) = node {
            list.push(parenthesize(input, new_node));
            return parenthesize(input, SyntaxTree::List(list));
        } else {
            panic!("expected ast node to be list but found {:?}", node);
        }
    } else if token == ")" {
        return node;
    } else {
        if let SyntaxTree::List(mut list) = node {
            list.push(categorize(&token));
            return parenthesize(input, SyntaxTree::List(list));
        } else {
            panic!("expected ast node to be list but found {:?}", node);
        }
    }
}

fn categorize(token: &String) -> SyntaxTree {
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
        value = Primitive::String(token.to_string());
    } else {
        value = Primitive::Identifier(token.to_string());
    };

    return SyntaxTree::Element(value);
}

fn parse(expression: String) -> SyntaxTree {
    let mut tokens = tokenize(expression);
    let root_node = SyntaxTree::List(Vec::new());
    let ast = parenthesize(&mut tokens, root_node);
    // println!("\n{:?}\n", ast);
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
