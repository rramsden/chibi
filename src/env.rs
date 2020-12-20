use super::types::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Scope {
    pub native_procedures: HashMap<String, Primitive>,
    pub variables: HashMap<String, Primitive>
}

pub fn standard_env() -> Scope {
    let mut native_procedures: HashMap<String, Primitive> = HashMap::new();
    native_procedures.insert("+".to_string(), Primitive::Function(addition));
    native_procedures.insert("-".to_string(), Primitive::Function(subtract));
    native_procedures.insert("*".to_string(), Primitive::Function(multiply));
    native_procedures.insert("/".to_string(), Primitive::Function(divide));
    native_procedures.insert(">".to_string(), Primitive::Function(greater_than));
    native_procedures.insert("<".to_string(), Primitive::Function(less_than));
    native_procedures.insert("=".to_string(), Primitive::Function(equals));
    native_procedures.insert("not".to_string(), Primitive::Function(not));

    let variables: HashMap<String, Primitive> = HashMap::new();

    Scope { native_procedures, variables }
}

fn falsy(v: &Primitive) -> bool {
    v == &Primitive::Bool(false) ||
    v == &Primitive::Integer(0) ||
    v == &Primitive::Nil
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

fn divide(list: Vec<Primitive>) -> Primitive {
    let leftmost = match list.first().unwrap() {
        Primitive::Integer(n) => f64::from(*n),
        Primitive::Float(n) => *n,
        n => panic!("can't device {:?}", n)
    };

    let result = list[1..].iter().fold(leftmost, |acc, x|
        match x {
            Primitive::Integer(n) => acc / f64::from(*n),
            Primitive::Float(n) => acc / n,
            _ => panic!("cannot divide {:?}", x)
        }
    );

    return Primitive::Float(result);
}

fn subtract(list: Vec<Primitive>) -> Primitive {
    let mut is_float = false;

    let leftmost = match list.first().unwrap() {
        Primitive::Integer(n) => f64::from(*n),
        Primitive::Float(n) => {
            is_float = true;
            *n
        },
        n => panic!("cant subtract {:?}", n)
    };

    if list.len() == 1 {
        if is_float {
            return Primitive::Float(-leftmost);
        } else {
            return Primitive::Integer(-(leftmost as i32));
        }
    }

    let result = list[1..].iter().fold(leftmost, |acc, x|
              match x {
                  Primitive::Integer(n) => acc - f64::from(*n),
                  Primitive::Float(n) => {
                      is_float = true;
                      acc - n
                  }
                  _ => panic!("cannot subtract {:?}", x)
              });

    if is_float {
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

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn test_subtract() {
        let a = Primitive::Integer(9);
        let b = Primitive::Integer(1);
        let list = vec![a, b];

        assert_eq!(subtract(list), Primitive::Integer(8));
    }

    #[test]
    fn test_division() {
        let a = Primitive::Integer(4);
        let b = Primitive::Float(2.0);
        let list = vec![a, b];

        assert_eq!(divide(list), Primitive::Float(2.0));
    }
}
