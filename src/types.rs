use std::fmt;
use super::env::Environment;

#[derive(Clone)]
pub enum Primitive {
    Identifier(String),
    String(String),
    Integer(i32),
    Float(f64),
    Tuple(Vec<Primitive>),
    Lambda(Lambda),
    Null
}

impl fmt::Debug for Primitive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Primitive::Identifier(i) => write!(f, "Identifier({})", i),
            Primitive::String(i) => write!(f, "String({})", i),
            Primitive::Integer(i) => write!(f, "Integer({})", i),
            Primitive::Float(i) => write!(f, "Float({})", i),
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
            Primitive::Lambda(i) => write!(f, "Lambda"),
            Primitive::Null => write!(f, "Null")
        }
    }
}

type Lambda = fn(Vec<Primitive>, &mut Environment) -> Primitive;

#[derive(Clone)]
pub enum SyntaxTree {
    Element(Primitive),
    List(Vec<SyntaxTree>)
}


fn format_tree(tree: &SyntaxTree, indent: i32, last: bool) -> String {
    let mut output = String::from("");

    match tree {
        SyntaxTree::Element(e) => {
            if last {
                format!("{:?}", e)
            } else {
                format!("{:?}, ", e)
            }
        },
        SyntaxTree::List(vec) => {
            for _ in 0..indent {
                output.push_str("  ");
            }
            for (pos, ast) in vec.iter().enumerate() {
                output.push_str(&format_tree(ast, indent + 1, pos == vec.len() - 1))
            }

            format!("List [\n  {}]", output)
        }

    }
}

impl fmt::Debug for SyntaxTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format_tree(self, 0, false))
    }
}
