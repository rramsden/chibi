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
                } else if leftmost == "cond" {
                    for clause in list[1..].into_iter() {
                        if let ParseTree::List(expressions) = clause {
                            let predicate = &expressions[0];
                            let expression = &expressions[1];
                            let (result, _) = interpret(predicate.clone(), scope.clone(), false);

                            if result == Primitive::Bool(true) {
                                return interpret(expression.clone(), scope.clone(), false);
                            }
                        }
                    }
                    panic!("case analysis failed to yield true result");
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
                match new_scope.stdlib.get(id) {
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

#[cfg(test)] 
mod tests {
    use super::interpret;
    use super::super::parser::parse;
    use super::super::env;
    use super::super::types::*;

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
}
