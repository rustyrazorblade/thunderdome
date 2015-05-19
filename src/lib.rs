#![allow(raw_pointer_derive)]
// #![feature(collections)]

pub mod graph;
pub mod vertex;
pub mod edge;
pub mod traversal;
pub mod path;
pub mod property;

#[cfg(test)]
mod tests {
    use vertex::RawVertex;
    use property::Property;
    use graph::{Graph, TraversableToVertex};
    use path::{Path, Element};






    #[test]
    fn test_get_vertex() {
        let mut g = Graph::new();
        let vertex = g.add_vertex();
        let v = g.get(1).unwrap();
        assert_eq!(v.id, 1);
    }





}
