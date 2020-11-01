mod env;
mod types;

use types::*;
use env::Environment;
use std::io::{self, Write, BufRead};

fn main() {
    // global lisp environment
    let mut scope = env::standard_env();

    loop {
        print!("> ");
        let expression = read();
        let result = interpret(parse(expression), &mut scope);

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

fn interpret(input: ParseTree, scope: &mut Environment) -> Primitive {
    match input {
        ParseTree::List(list) if list.len() > 0 => {
            if let ParseTree::Element(Primitive::Identifier(leftmost)) = &list[0] {
                if leftmost == "define" && list.len() == 3 {
                    let arguments = list[1].clone();
                    let body = list[2].clone();

                    if let ParseTree::Element(Primitive::Identifier(id)) = arguments {
                        let result = interpret(body, scope);
                        return define_constant(id, result, scope);
                    } else if let ParseTree::List(signature) = arguments {
                        return define_function(signature, body, scope);
                    }
                } else if let Some((signature, body)) = scope.clone().functions.get(leftmost) {
                    let mut params: Vec<Primitive> = vec![];
                    for param in list[1..].into_iter() {
                        params.push( interpret(param.clone(), scope).clone() )
                    }

                    return apply(signature.to_vec(), params, body.clone(), scope);
                }
            }

            // map over and eval all elements in list
            let slice: Vec<Primitive> = list.into_iter().map(|tree| interpret(tree, scope)).collect();

            if let Primitive::Identifier(id) = slice.first().unwrap() {
                match scope.stdlib.get(id) {
                    Some(Primitive::Lambda(lambda)) => return lambda(slice[1..].to_vec(), scope),
                    _ => return Primitive::Tuple(slice)
                }
            } else {
                if slice.len() == 1 {
                    slice[0].clone()
                } else {
                    Primitive::Tuple(slice)
                }
            }
        },
        ParseTree::List(_) => Primitive::Tuple(vec![]), // empty case
        ParseTree::Element(primitive) => {
            match primitive {
                Primitive::Identifier(id) => {
                    match scope.variables.get(&id) {
                        Some(primitive) => return primitive.clone(),
                        _ => return Primitive::Identifier(id)
                    }
                }
                _ => primitive
            }
        }
    }
}

fn define_constant(label: String, value: Primitive, scope: &mut Environment) -> Primitive {
    scope.variables.insert(label.clone(), value);
    return Primitive::Identifier(label);
}

fn define_function(signature: Vec<ParseTree>, body: ParseTree, scope: &mut Environment) -> Primitive {
    let slice = &signature[..];
    let label = &slice[0];
    let arguments: Vec<ParseTree> = slice[1..].into();

    if let ParseTree::Element(Primitive::Identifier(id)) = label {
        scope.functions.insert(id.clone(), (arguments, body));
        return Primitive::Identifier(id.clone());
    } else {
        panic!("expected unknown type");
    }
}

fn apply(signature: Vec<ParseTree>, values: Vec<Primitive>, body: ParseTree, scope: &mut Environment) -> Primitive {
    for (i, id) in signature.into_iter().enumerate() {
        if let ParseTree::Element(Primitive::Identifier(varname)) = id {
            scope.variables.insert(varname, values[i].clone());
        }
    }

    return interpret(body, scope);
}

/// Builds an abstract syntax tree from tokenized input and returns a ParseTree
fn parenthesize(input: &mut Vec<String>, node: ParseTree) -> ParseTree {
    if input.len() == 0 {
        return node
    }

    let token = input.remove(0);
   
    if token == "(" {
        let new_node = ParseTree::List(Vec::new());

        if let ParseTree::List(mut list) = node {
            list.push(parenthesize(input, new_node));
            return parenthesize(input, ParseTree::List(list));
        } else {
            panic!("expected ast node to be list but found {:?}", node);
        }
    } else if token == ")" {
        return node;
    } else {
        if let ParseTree::List(mut list) = node {
            list.push(categorize(&token));
            return parenthesize(input, ParseTree::List(list));
        } else {
            panic!("expected ast node to be list but found {:?}", node);
        }
    }
}

fn categorize(token: &String) -> ParseTree {
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

    return ParseTree::Element(value);
}

fn parse(expression: String) -> ParseTree {
    let mut tokens = tokenize(expression);
    let root_node = ParseTree::List(Vec::new());
    let parse_tree = parenthesize(&mut tokens, root_node);
    println!("\n{:?}\n", parse_tree);
    return parse_tree;
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
