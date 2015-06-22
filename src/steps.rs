/*
returned from the peg parser
we'll need to take each of the steps
and use them to construct an actual GraphQuery
*/
pub struct ParsedGraphQuery {
    pub scope: Scope,
    pub steps: Vec<Step>
}

pub enum Scope {
    Global,
    Vertex(Vec<i64>),
}

pub struct Step {
    pub name: String
}

pub enum Arg {
    Integer(i64),
    Float(f64),
    String(String),
}

impl Step {
    pub fn new(name: String) -> Step {
        Step{name:name}
    }
}
