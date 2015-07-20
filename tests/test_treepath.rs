extern crate thunderdome;

use thunderdome::graph::Graph;
use thunderdome::vertex::Vertex;
use thunderdome::path::Element;
use thunderdome::treepath::TreePath;

fn simple_graph() -> Box<Graph> {
    let mut g = Graph::new();
    let mut jon = g.add_vertex();
    let mut blake = g.add_vertex();
    let mut caleb = g.add_vertex();
    let mut mike = g.add_vertex();

    jon.add_edge(&mut blake, "friend");
    jon.add_edge(&mut caleb, "friend");
    blake.add_edge(&mut caleb, "friend");
    blake.add_edge(&mut jon, "tried_to_murder");

    jon.add_edge(&mut mike, "enemy");
    g
}

fn sample_tree() -> TreePath {
    let t = TreePath::new();
    // t.add_child()
    t
}

#[test]
fn test_create_tree() {

    let mut g = Graph::new();
    let v = g.add_vertex();
    let v2 = g.add_vertex();
    let v3 = g.add_vertex();

    let mut t = TreePath::from_vertex(v);
    t.add_child(Element::Vertex(v2));
    assert_eq!(1, t.element_count);

    t.add_child(Element::Vertex(v3));

    assert_eq!(2, t.child_count());

}

#[test]
fn test_iterate_over_leaves() {
    // make sure we can iterate over the tree's leaves and remove sub trees
}
