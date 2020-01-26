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
struct Node {
    is_list: bool,
    element: Option<Element>,
    list: Vec<Node>
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

fn tokenize(expression: String) -> Vec<String> {
    let expression = expression 
        .replace("(", " ( ")
        .replace(")", " ) ")
        .trim()
        .to_string();

    expression.split_whitespace().map(|s| s.to_string()).collect()
}

fn parenthesize(mut input: Vec<String>, mut node: Node) -> Node {
    if input.len() == 0 {
        return node
    }

    let token = input.remove(0);
    
    if token == "(" {
        let new_node = Node {
            is_list: true,
            element: None,
            list: Vec::new()
        };

        node.list.push(parenthesize(input.clone(), new_node));
        return node;
    } else if token == ")" {
        return node;
    } else {
        node.list.push(categorize(token));
        return parenthesize(input.clone(), node);
    }
}

fn categorize(token: String) -> Node {
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

    return Node {
        is_list: false,
        element: Some(Element { value: token, kind: kind }),
        list: Vec::new()
    }
}

fn parse(expression: String) -> Node {
    let tokens = tokenize(expression);
    let node = Node {
        is_list: true,
        element: None,
        list: Vec::new()
    };

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
