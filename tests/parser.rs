

extern crate thunderdome;



#[cfg(test)]
mod parser_tests {
    use thunderdome::parser::*;
    use thunderdome::steps::*;

    fn validate(q: &str) -> Option<GraphQuery> {
        let result = parse(q);
        assert!(result.is_ok());
        result.ok()
    }

    #[test]
    fn global_graph_query() {
        validate("g.V");
    }

    #[test]
    fn vertex_query() {
        validate("g.v(1)");
        validate("g.v(1,2)");
        validate("g.v(1, 2)");
    }

    #[test]
    fn simple_step_test() {
        validate("g.v(1).outV()");
        validate("g.v(1).outV().inE()");
    }

    #[test]
    fn test_args() {
        validate("g.v(1).outV('edge').has('age', 30)");
    }
}
