use std::fmt;

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
struct ElementTree {
    is_list: bool,
    element: Option<Element>,
    list: Vec<ElementTree>
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

fn parenthesize(mut input: Vec<String>, mut value_or_list: ElementTree) -> ElementTree {
    if input.len() == 0 {
        return value_or_list
    }

    let token = input.remove(0);
    
    if token == "(" {
        let node = ElementTree {
            is_list: true,
            element: None,
            list: Vec::new()
        };

        value_or_list.list.push(parenthesize(input.clone(), node));
        return value_or_list;
    } else if token == ")" {
        return value_or_list;
    } else {
        value_or_list.list.push(categorize(token));
        return parenthesize(input.clone(), value_or_list);
    }
}

fn categorize(token: String) -> ElementTree {
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

    return ElementTree {
        is_list: false,
        element: Some(Element { value: token, kind: kind }),
        list: Vec::new()
    }
}

fn parse(expression: String) -> ElementTree {
    let tokens = tokenize(expression);
    let value_or_list = ElementTree {
        is_list: true,
        element: None,
        list: Vec::new()
    };

    return parenthesize(tokens, value_or_list);
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
