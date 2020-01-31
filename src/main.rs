#[derive(Debug)]
enum ElementKind {
    IDENTIFIER,
    LITERAL,
}

#[derive(Debug)]
struct Element {
    value: String,
    kind: ElementKind 
}

#[derive(Debug)]
enum AST {
    Element(Element),
    List(Vec<AST>)
}

use std::io::{self, Write, BufRead};

fn main() {
    loop {
        print!("> ");
        let expression = read();
        let result = parse(expression);

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

/// Builds an abstract syntax tree from tokenized input and returns an AST
fn parenthesize(mut input: Vec<String>, node: AST) -> AST {
    if input.len() == 0 {
        return node
    }

    let token = input.remove(0);
    
    if token == "(" {
        let new_node = AST::List(Vec::new());

        if let AST::List(mut list) = node {
            list.push(parenthesize(input.clone(), new_node));
            return AST::List(list);
        } else {
            return node;
        }
    } else if token == ")" {
        return node;
    } else {
        if let AST::List(mut list) = node {
            list.push(categorize(token));
            return parenthesize(input.clone(), AST::List(list));
        } else {
            return node;
        }
    }
}

fn categorize(token: String) -> AST {
    let first_ch = token.chars().next().unwrap();
    let last_ch = token.chars().last().unwrap();

    let kind: ElementKind;
    if token.parse::<f64>().is_ok() {
        kind = ElementKind::LITERAL;
    } else if first_ch == '"' && last_ch == '"' {
        kind = ElementKind::LITERAL;
    } else {
        kind = ElementKind::IDENTIFIER;
    };

    return AST::Element(Element { value: token, kind: kind });
}

fn parse(expression: String) -> AST {
    let tokens = tokenize(expression);
    let node = AST::List(Vec::new());
    return parenthesize(tokens, node);
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
