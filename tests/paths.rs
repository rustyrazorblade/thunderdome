extern crate thunderdome;

#[cfg(test)]
mod path_tests {
    use thunderdome::graph::*;
    use thunderdome::path::*;

    #[test]
    fn test_permute_path() {
        let mut g = Graph::new();
        let mut v = g.add_vertex();
        let mut v2 = g.add_vertex();
        let mut v3 = g.add_vertex();

        let vec = vec![Element::Vertex(v2),
                       Element::Vertex(v3)];

        let mut path = Path::new(v);

        // check last path
        let path2 = path.permute(&vec);
    }
}
