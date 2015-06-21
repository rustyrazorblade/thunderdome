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
    pub fn new(name: &str) -> Step {
        Step{name:name.to_string()}
    }
}
