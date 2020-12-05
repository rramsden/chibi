use super::types::*;
use super::env::Scope;

pub fn interpret(input: ParseTree, scope: Scope, global: bool) -> (Primitive, Scope) {
    match input {
        ParseTree::List(list) if list.len() > 0 => {
            if let ParseTree::Element(Primitive::Identifier(leftmost)) = &list[0] {
                if leftmost == "define" && list.len() == 3 {
                    let arguments = list[1].clone();
                    let body = list[2].clone();

                    if let ParseTree::Element(Primitive::Identifier(id)) = arguments {
                        let (result, new_scope) = interpret(body, scope, false);
                        return define_constant(id, result, new_scope);
                    } else if let ParseTree::List(signature) = arguments {
                        return define_procedure(signature, body, scope);
                    }
                } else if leftmost == "if" {
                    let predicate = list[1].clone();
                    let consequent = list[2].clone();
                    let alternative = list[3].clone();

                    let (result, _) = interpret(predicate.clone(), scope.clone(), false);

                    if result == Primitive::Bool(true) {
                        return interpret(consequent, scope.clone(), false);
                    } else {
                        return interpret(alternative, scope.clone(), false);
                    }
                } else if leftmost == "and" {
                    let mut last_result: Primitive = Primitive::Bool(true);

                    for expression in list[1..].into_iter() {
                        let (result, _) = interpret(expression.clone(), scope.clone(), false);
                        last_result = result;

                        if falsy(&last_result) {
                            return (Primitive::Bool(false), scope);
                        }
                    }

                    return (last_result, scope);
                } else if leftmost == "or" {
                    let mut last_result: Primitive;

                    for expression in list[1..].into_iter() {
                        let (result, _) = interpret(expression.clone(), scope.clone(), false);
                        last_result = result;

                        if truthy(&last_result) {
                            return (last_result, scope);
                        }
                    }

                    return (Primitive::Nil, scope);
                } else if leftmost == "cond" {
                    for clause in list[1..].into_iter() {
                        if let ParseTree::List(expressions) = clause {
                            let predicate = &expressions[0];
                            let expression = &expressions[1];
                            let (result, _) = interpret(predicate.clone(), scope.clone(), false);

                            // evaluate if predicate is true or "else" is present
                            // e.g. ((cond (else something))
                            if result == Primitive::Bool(true) ||
                               result == Primitive::Identifier(String::from("else")) {
                                return interpret(expression.clone(), scope.clone(), false);
                            }
                        }
                    }
                    return (Primitive::Nil, scope);
                } else if let Some((signature, body)) = scope.clone().procedures.get(leftmost) {
                    let mut params: Vec<Primitive> = vec![];

                    for param in list[1..].into_iter() {
                        let (result, _) = interpret(param.clone(), scope.clone(), false).clone();
                        params.push( result )
                    }

                    return apply(signature.to_vec(), params, body.clone(), scope);
                }
            }

            let mut new_scope = scope.clone();
            let mut results: Vec<Primitive> = vec![];

            for element in list {
                let (result, updated_scope) = interpret(element, new_scope, false);
                results.push(result);
                new_scope = updated_scope;
            }

            let last_result = results.last().unwrap().clone();

            if !global {
                new_scope = scope;
            }

            if let Primitive::Identifier(id) = results.first().unwrap() {
                match new_scope.native_procedures.get(id) {
                    Some(Primitive::Lambda(lambda)) => return (lambda(results[1..].to_vec()), new_scope),
                    _ => return (last_result, new_scope)
                }
            } else {
                if results.len() == 1 {
                    return (results[0].clone(), new_scope)
                } else {
                    return (last_result, new_scope)
                }
            }
        },
        ParseTree::List(_) => (Primitive::Tuple(vec![]), scope), // empty case
        ParseTree::Element(primitive) => {
            match primitive {
                Primitive::Identifier(id) => {
                    match scope.variables.get(&id) {
                        Some(primitive) => return (primitive.clone(), scope),
                        _ => return (Primitive::Identifier(id), scope)
                    }
                }
                _ => return (primitive, scope)
            }
        }
    }
}

fn define_constant(label: String, value: Primitive, mut scope: Scope) -> (Primitive, Scope) {
    scope.variables.insert(label.clone(), value);
    return (Primitive::Identifier(label), scope);
}

fn define_procedure(signature: Vec<ParseTree>, body: ParseTree, mut scope: Scope) -> (Primitive, Scope) {
    let slice = &signature[..];
    let name = &slice[0];
    let formal_arguments: Vec<ParseTree> = slice[1..].into();

    if let ParseTree::Element(Primitive::Identifier(id)) = name {
        scope.procedures.insert(id.clone(), (formal_arguments, body));
        return (Primitive::Identifier(id.clone()), scope);
    } else {
        panic!("expected unknown type");
    }
}

fn apply(signature: Vec<ParseTree>, values: Vec<Primitive>, body: ParseTree, mut scope: Scope) -> (Primitive, Scope) {
    for (i, id) in signature.into_iter().enumerate() {
        if let ParseTree::Element(Primitive::Identifier(varname)) = id {
            scope.variables.insert(varname, values[i].clone());
        }
    }

    return interpret(body, scope, false);
}

fn falsy(v: &Primitive) -> bool {
    v == &Primitive::Bool(false) ||
    v == &Primitive::Integer(0) ||
    v == &Primitive::Nil
}

fn truthy(v: &Primitive) -> bool {
    !falsy(v)
}

#[cfg(test)] 
mod tests {
    use super::interpret;
    use super::super::parser::parse;
    use super::super::env;
    use super::super::types::*;

    #[test]
    fn test_case_and_empty() {
        let scope = env::standard_env();
        let parse_tree = parse("(and)");
        let (result, _) = interpret(parse_tree, scope, true);
        assert_eq!(result, Primitive::Bool(true));
    }

    #[test]
    fn test_case_and_return_last_truthy() {
        let scope = env::standard_env();
        let parse_tree = parse("(and 1 2 3 \"Eureka\")");
        let (result, _) = interpret(parse_tree, scope, true);
        assert_eq!(result, Primitive::String(String::from("Eureka")));
    }

    #[test]
    fn test_case_and_return_false() {
        let scope = env::standard_env();
        let parse_tree = parse("(and 1 2 3 4 0)");
        let (result, _) = interpret(parse_tree, scope, true);
        assert_eq!(result, Primitive::Bool(false));
    }

    #[test]
    fn test_case_or_empty() {
        let scope = env::standard_env();
        let parse_tree = parse("(or)");
        let (result, _) = interpret(parse_tree, scope, true);
        assert_eq!(result, Primitive::Nil);
    }

    #[test]
    fn test_case_or_return_first_truthy() {
        let scope = env::standard_env();
        let parse_tree = parse("(or 0 0 1 0)");
        let (result, _) = interpret(parse_tree, scope, true);
        assert_eq!(result, Primitive::Integer(1));
    }

    #[test]
    fn test_case_or_return_nil() {
        let scope = env::standard_env();
        let parse_tree = parse("(or 0 0 0 0)");
        let (result, _) = interpret(parse_tree, scope, true);
        assert_eq!(result, Primitive::Nil);
    }

    #[test]
    fn test_case_analysis() {
        let scope = env::standard_env();
        let parse_tree = parse("
            (define (abs x)
                (cond ((> x 0) x)
                      ((= x 0) 0)
                      ((< x 0) (- x))))

            (abs -5)");

        let (result, _) = interpret(parse_tree, scope, true);
        assert_eq!(result, Primitive::Integer(5));
    }

    #[test]
    fn test_else_case_analysis() {
        let scope = env::standard_env();
        let parse_tree = parse("
            (cond (= 1 0)
                  (else 123))
        ");

        let (result, _) = interpret(parse_tree, scope, true);
        assert_eq!(result, Primitive::Integer(123));
    }

    #[test]
    fn test_case_analysis_undefined() {
        let scope = env::standard_env();
        let parse_tree = parse("(cond (= 1 2))");
        let (result, _) = interpret(parse_tree, scope, true);
        assert_eq!(result, Primitive::Nil);
    }

    #[test]
    fn test_if_statement() {
        let scope = env::standard_env();
        let parse_tree = parse("
            (define (abs x)
                (if (< x 0)
                    (- x)
                    x))
            (abs -10)
        ");

        let (result, _) = interpret(parse_tree, scope, true);
        assert_eq!(result, Primitive::Integer(10));
    }
}
