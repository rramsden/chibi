use std::fmt;

#[derive(Clone)]
pub enum ParseTree {
    Element(Primitive),
    List(Vec<ParseTree>)
}

impl PartialEq for ParseTree {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ParseTree::List(vec1), ParseTree::List(vec2)) => vec1 == vec2,
            (ParseTree::Element(e1), ParseTree::Element(e2)) => e1 == e2,
            _ => false
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Primitive {
    Identifier(String),
    String(String),
    Integer(i32),
    Float(f64),
    Bool(bool),
    Tuple(Vec<Primitive>),
    Lambda(Lambda)
}

impl fmt::Debug for Primitive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Primitive::Identifier(i) => write!(f, "Identifier({})", i),
            Primitive::String(i) => write!(f, "String({})", i),
            Primitive::Integer(i) => write!(f, "Integer({})", i),
            Primitive::Float(i) => write!(f, "Float({})", i),
            Primitive::Bool(i) => write!(f, "Bool({})", i),
            Primitive::Tuple(vec) => {
                let mut out = String::from("");
                for (pos, p) in vec.iter().enumerate() {
                    if pos == vec.len() - 1 {
                        out.push_str(&format!("{:?}", p));
                    } else {
                        out.push_str(&format!("{:?}, ", p));
                    }
                }
                write!(f, "Tuple({})", out)
            },
            Primitive::Lambda(_) => write!(f, "Lambda")
        }
    }
}

type Lambda = fn(Vec<Primitive>) -> Primitive;

impl fmt::Debug for ParseTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format_tree(self, 0))
    }
}

fn format_tree(tree: &ParseTree, indent: i32) -> String {
    let mut output = String::from("");

    match tree {
        ParseTree::Element(e) => { format!("{:?}", e) },
        ParseTree::List(vec) => {
            for _ in 0..indent { output.push_str("  "); }

            for (pos, ast) in vec.iter().enumerate() {
                output.push_str(&format_tree(ast, indent + 1));

                let end_of_array = pos == vec.len() - 1;
                if !end_of_array {
                    output.push_str(", ");
                }
            }

            format!("List [\n  {}]", output)
        }

    }
}
