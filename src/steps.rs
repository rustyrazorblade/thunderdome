pub struct GraphQuery {
    pub blah: i64,
    pub scope: Scope,
}

pub enum Scope {
    Global,
    Vertex(Vec<i64>),
}
