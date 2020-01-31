#[derive(Debug, Clone)]
pub enum Primitive {
    Identifier(String),
    String(String),
    Integer(i32),
    Float(f64),
    Tuple(Vec<Primitive>),
    Null
}

#[derive(Debug, Clone)]
pub enum SyntaxTree {
    Element(Primitive),
    List(Vec<SyntaxTree>)
}

#[derive(Debug, Clone)]
pub struct Context {
    pub scope: Vec<Primitive>
}

impl Context {
    fn get(&self, id: String) -> Primitive {
        let result = self.scope.iter().find(|x|
                               match x {
                                   Primitive::Identifier(a) => *a == id,
                                   _ => false
                               });

        match result {
            Some(v) => v.clone(),
            None => Primitive::Null
        }
    }
}
