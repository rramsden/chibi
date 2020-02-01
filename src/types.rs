use super::env::Environment;

#[derive(Debug, Clone)]
pub enum Primitive {
    Identifier(String),
    String(String),
    Integer(i32),
    Float(f64),
    Tuple(Vec<Primitive>),
    Lambda(Lambda),
    Null
}

type Lambda = fn(Vec<Primitive>, Environment) -> Primitive;

#[derive(Debug, Clone)]
pub enum SyntaxTree {
    Element(Primitive),
    List(Vec<SyntaxTree>)
}
