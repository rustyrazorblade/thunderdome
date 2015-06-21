pub struct GraphQuery {
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

impl Step {
    pub fn new(name: String) -> Step {
        Step{name:name}
    }
}
