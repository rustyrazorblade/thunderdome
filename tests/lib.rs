extern crate thunderdome;

#[cfg(test)]
mod helpers {
    use thunderdome::graph::Graph;

    fn toy_graph_friends() -> Box<Graph> {
        let mut g = Graph::new();
        let mut v1 = g.add_vertex();
        g
    }

}
