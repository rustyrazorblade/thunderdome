extern crate thunderdome;

use thunderdome::graph::Graph;
use thunderdome::vertex::Vertex;
use thunderdome::path::Element;
use thunderdome::treepath::TreePath;


#[test]
fn test_create_tree() {

    let mut g = Graph::new();
    let v = g.add_vertex();
    let v2 = g.add_vertex();
    let v3 = g.add_vertex();

    let mut t = TreePath::from_vertex(v);
    t.add_child(Element::Vertex(v2));
    t.add_child(Element::Vertex(v3));

    assert_eq!(2, t.child_count());

}
