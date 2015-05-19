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
    fn test_unsafe_vertex() {
        let v = RawVertex::new(1);

        let out_edges = &v.out_edges;
        let in_edges = &v.in_edges;
    }

    #[test]
    fn test_deref() {
        let mut g = Graph::new();
        let v1 = g.add_vertex();
        let vertex = &*v1;
        assert!(vertex.id == 1);
    }

    #[test]
    fn test_new_vertex() {
        let mut g = Graph::new();
        let v1 = g.add_vertex();

        assert_eq!(v1.id, 1);

        unsafe {
            let vertex = &*v1;
            assert_eq!((*v1).id, 1);
        }

        let v2 = g.add_vertex();
        assert!(v2.id == 2);
    }




    #[test]
    fn test_get_vertex() {
        let mut g = Graph::new();
        let vertex = g.add_vertex();
        let v = g.get(1).unwrap();
        assert_eq!(v.id, 1);
    }





}
