extern crate thunderdome;

mod tree_path_tests {
    use thunderdome::treepath::TreePath;
    use thunderdome::graph::*;

    #[test]
    fn create_tree_from_element() {

        let mut g = Graph::new();
        let mut v = g.add_vertex();

        TreePath::from_vertex(v);

    }
}
