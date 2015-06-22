extern crate thunderdome;


#[cfg(test)]
mod parser_tests {
    use thunderdome::parser::*;
    use thunderdome::steps::*;

    fn validate(q: &str) -> Option<ParsedGraphQuery> {
        let result = pre_parse(q);
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
        let result = validate("g.v(1).outV().inE()").unwrap();
        assert_eq!(result.steps.len(), 2);
        assert_eq!(result.steps.get(0).unwrap().name, "outV".to_string());
        assert_eq!(result.steps.get(1).unwrap().name, "inE".to_string());
    }

    #[test]
    fn test_args() {
        validate("g.v(1).outV('edge').has('age', 30)");
    }
    #[test]
    fn test_args_numbers() {
        // maybe a weird offset command?
        validate("g.V.limit(10, 20)");
        validate("g.V.limit(10.0)");
    }
}
