extern crate thunderdome;

#[cfg(test)]
mod helpers {
    use thunderdome::graph::Graph;

    fn toy_graph_friends() -> Box<Graph> {
        let mut g = Graph::new();
        let mut jon = g.add_vertex();
        let mut josh = g.add_vertex();
        let mut sean = g.add_vertex();
        g
    }

}
