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
    stdlib.insert(">=".to_string(), Primitive::Lambda(greater_than_equals));
    stdlib.insert("<=".to_string(), Primitive::Lambda(less_than_equals));
    stdlib.insert("<".to_string(), Primitive::Lambda(less_than));
    stdlib.insert("=".to_string(), Primitive::Lambda(equals));

    let variables: HashMap<String, Primitive> = HashMap::new();
    let procedures: HashMap<String, (Vec<ParseTree>, ParseTree)> = HashMap::new();

    Scope { stdlib, variables, procedures }
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

fn greater_than_equals(list: Vec<Primitive>) -> Primitive {
    if greater_than(list.clone()) == Primitive::Bool(true) {
        return Primitive::Bool(true);
    } else if equals(list.clone()) == Primitive::Bool(true) {
        return Primitive::Bool(true);
    } else {
        return Primitive::Bool(false);
    }
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

fn less_than_equals(list: Vec<Primitive>) -> Primitive {
    if less_than(list.clone()) == Primitive::Bool(true) {
        return Primitive::Bool(true);
    } else if equals(list.clone()) == Primitive::Bool(true) {
        return Primitive::Bool(true);
    } else {
        return Primitive::Bool(false);
    }
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
