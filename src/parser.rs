use super::types::*;

pub fn parse(expression: String) -> ParseTree {
    let mut tokens = tokenize(expression);
    let root_node = ParseTree::List(Vec::new());
    let parse_tree = parenthesize(&mut tokens, root_node);
    return parse_tree;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let parse_tree = ParseTree::List(vec![]);
        assert_eq!(parse(String::from("(+ 1 1)")), parse_tree);
    }
}
