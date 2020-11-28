use super::types::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Scope {
    pub stdlib: HashMap<String, Primitive>,
    pub variables: HashMap<String, Primitive>,

    // label, (signature, body)
    pub procedures: HashMap<String, (Vec<ParseTree>, ParseTree)>
}

pub fn standard_env() -> Scope {
    let mut stdlib: HashMap<String, Primitive> = HashMap::new();
    stdlib.insert("+".to_string(), Primitive::Lambda(addition));
    stdlib.insert("-".to_string(), Primitive::Lambda(subtract));
    stdlib.insert("*".to_string(), Primitive::Lambda(multiply));
    stdlib.insert(">".to_string(), Primitive::Lambda(greater_than));
    stdlib.insert("<".to_string(), Primitive::Lambda(less_than));
    stdlib.insert("=".to_string(), Primitive::Lambda(equals));
    stdlib.insert("not".to_string(), Primitive::Lambda(not));

    let variables: HashMap<String, Primitive> = HashMap::new();
    let procedures: HashMap<String, (Vec<ParseTree>, ParseTree)> = HashMap::new();

    Scope { stdlib, variables, procedures }
}

fn falsy(v: &Primitive) -> bool {
    v == &Primitive::Bool(false) ||
    v == &Primitive::Integer(0) ||
    v == &Primitive::Null
}

fn truthy(v: &Primitive) -> bool {
    !falsy(v)
}

fn not(list: Vec<Primitive>) -> Primitive {
    if truthy(&list[0]) {
        return Primitive::Bool(false);
    } else {
        return Primitive::Bool(true);
    }
}

fn greater_than(list: Vec<Primitive>) -> Primitive {
    let left = &list[0];
    let right = &list[1];

    if let Primitive::Integer(a1) = left {
        if let Primitive::Integer(a2) = right {
            return Primitive::Bool(a1 > a2);
        }
    }

    return Primitive::Bool(false);
}

fn equals(list: Vec<Primitive>) -> Primitive {
    let left = &list[0];
    let right = &list[1];

    if let Primitive::Integer(a1) = left {
        if let Primitive::Integer(a2) = right {
            return Primitive::Bool(a1 == a2);
        }
    }

    return Primitive::Bool(false);
}

fn less_than(list: Vec<Primitive>) -> Primitive {
    let left = &list[0];
    let right = &list[1];

    if let Primitive::Integer(a1) = left {
        if let Primitive::Integer(a2) = right {
            return Primitive::Bool(a1 < a2);
        }
    }

    return Primitive::Bool(false);
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
