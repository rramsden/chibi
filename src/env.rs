use super::types::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    pub stdlib: HashMap<String, Primitive>,
    pub variables: HashMap<String, Primitive>,

    // label, (signature, body)
    pub functions: HashMap<String, (Vec<ParseTree>, ParseTree)>
}

pub fn standard_env() -> Environment {
    let mut stdlib: HashMap<String, Primitive> = HashMap::new();
    stdlib.insert("+".to_string(), Primitive::Lambda(addition));
    stdlib.insert("-".to_string(), Primitive::Lambda(subtract));
    stdlib.insert("*".to_string(), Primitive::Lambda(multiply));

    let variables: HashMap<String, Primitive> = HashMap::new();
    let functions: HashMap<String, (Vec<ParseTree>, ParseTree)> = HashMap::new();

    Environment { stdlib, variables, functions }
}

fn addition(list: Vec<Primitive>) -> Primitive {
    let mut has_float = false;
    let result = list.iter().fold(0f64, |acc, x|
              match x {
                  Primitive::Integer(n) => f64::from(*n) + acc,
                  Primitive::Float(n) => {
                      has_float = true;
                      n + acc
                  },
                  _ => panic!("cannot add {:?}", x)
              });
    if has_float {
        Primitive::Float(result)
    } else {
        Primitive::Integer(result as i32)
    }
}

fn subtract(list: Vec<Primitive>) -> Primitive {
    let mut has_float = false;
    let result = list.iter().fold(0f64, |acc, x|
              match x {
                  Primitive::Integer(n) => acc - f64::from(*n),
                  Primitive::Float(n) => {
                      has_float = true;
                      acc - n
                  }
                  _ => panic!("cannot subtract {:?}", x)
              });
    if has_float {
        Primitive::Float(result)
    } else {
        Primitive::Integer(result as i32)
    }
}

fn multiply(list: Vec<Primitive>) -> Primitive {
    let mut has_float = false;
    let result = list.iter().fold(1f64, |acc, x|
              match x {
                  Primitive::Integer(n) => acc * f64::from(*n),
                  Primitive::Float(n) => {
                      has_float = true;
                      acc * n
                  }
                  _ => panic!("cannot multiply {:?}", x)
              });
    if has_float {
        Primitive::Float(result)
    } else {
        Primitive::Integer(result as i32)
    }
}
