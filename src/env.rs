use super::types::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    pub stdlib: HashMap<String, Primitive>,
    pub definitions: HashMap<String, Primitive>
}

pub fn standard_env() -> Environment {
    let mut stdlib: HashMap<String, Primitive> = HashMap::new();
    stdlib.insert("+".to_string(), Primitive::Lambda(addition));
    stdlib.insert("-".to_string(), Primitive::Lambda(subtract));
    stdlib.insert("*".to_string(), Primitive::Lambda(multiply));
    stdlib.insert("define".to_string(), Primitive::Lambda(define));

    let definitions: HashMap<String, Primitive> = HashMap::new();

    Environment { stdlib: stdlib, definitions: definitions }
}

fn define(list: Vec<Primitive>, env: &mut Environment) -> Primitive {
    println!("list: {:?}", list);
    let first = list.first().unwrap();

    if let Primitive::Identifier(id) = first {
        env.definitions.insert(id.to_string(), list[1].clone());
        return first.clone();
    } else {
        panic!("define needs type Identifier, received {:?}", first);
    }
}

fn addition(list: Vec<Primitive>, _: &mut Environment) -> Primitive {
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

fn subtract(list: Vec<Primitive>, _: &mut Environment) -> Primitive {
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

fn multiply(list: Vec<Primitive>, _: &mut Environment) -> Primitive {
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
