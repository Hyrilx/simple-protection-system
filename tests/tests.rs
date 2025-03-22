#[cfg(test)]
mod tests {
    use simple_protection_system::graph::Graph;
    use simple_protection_system::right::Right;

    #[test]
    fn init_graph() {
        let mut graph = Graph::new();
        assert_eq!(graph.objects().len(), 0);
        assert_eq!(graph.capabilities().len(), 0);

        let a = graph.new_object();

        assert_eq!(graph.objects().len(), 1);
        assert_eq!(graph.capabilities().len(), 0);

        assert!(graph.contains_object(&a));
    }

    #[test]
    fn rule_create() {
        let mut graph = Graph::new();
        let root = graph.new_object();
        let root_a = graph.create(&root);
        let _root_b = graph.create(&root);
        let root_c = graph.create(&root);
        let _root_a_a = graph.create(&root_a);
        let root_c_a = graph.create(&root_c);
        let _root_c_a_a = graph.create(&root_c_a);

        assert_eq!(graph.objects().len(), 7);
        assert_eq!(graph.capabilities().len(), 6);
    }

    #[test]
    fn rule_remove() {
        let mut graph = Graph::new();
        let root = graph.new_object();
        let root_a = graph.create(&root);
        let _root_b = graph.create(&root);
        let root_c = graph.create(&root);
        let _root_a_a = graph.create(&root_a);
        let root_c_a = graph.create(&root_c);
        let root_c_a_a = graph.create(&root_c_a);

        graph.remove(&root, &root_a);
        graph.remove(&root_c, &root_c_a);

        graph.remove(&root, &root_c_a_a);

        assert_eq!(graph.objects().len(), 7);
        assert_eq!(graph.capabilities().len(), 4);
    }

    #[test]
    fn rule_take_success() {
        let mut graph = Graph::new();
        let root = graph.new_object();
        let root_a = graph.create(&root);
        let _root_b = graph.create(&root);
        let root_c = graph.create(&root);
        let root_a_a = graph.create(&root_a);
        let root_c_a = graph.create(&root_c);
        let root_c_a_a = graph.create(&root_c_a);

        assert!(!graph.have_operation(&root, &root_a_a, &Right::read()));
        assert!(!graph.have_operation(&root, &root_c_a, &Right::read()));
        assert!(!graph.have_operation(&root, &root_c_a_a, &Right::read()));
        assert!(!graph.have_operation(&root_c, &root_c_a_a, &Right::read()));

        graph.take(&root, &root_a_a);
        graph.take(&root, &root_c_a);
        graph.take(&root, &root_c_a_a);
        graph.take(&root_c, &root_c_a_a);

        assert!(graph.have_operation(&root, &root_a_a, &Right::read()));
        assert!(graph.have_operation(&root, &root_c_a, &Right::read()));
        assert!(graph.have_operation(&root, &root_c_a_a, &Right::read()));
        assert!(graph.have_operation(&root_c, &root_c_a_a, &Right::read()));
    }

    #[test]
    fn rule_take_fail() {
        let mut graph = Graph::new();
        let root = graph.new_object();
        let root_a = graph.create(&root);
        let root_b = graph.create(&root);
        let root_c = graph.create(&root);
        let root_a_a = graph.create(&root_a);
        let root_c_a = graph.create(&root_c);
        let root_c_a_a = graph.create(&root_c_a);

        assert!(!graph.have_operation(&root, &root_c_a_a, &Right::read()));
        assert!(!graph.have_operation(&root_b, &root_a_a, &Right::read()));

        graph.take(&root, &root_c_a_a);
        graph.take(&root_b, &root_a_a);

        assert!(!graph.have_operation(&root, &root_c_a_a, &Right::read()));
        assert!(!graph.have_operation(&root_b, &root_a_a, &Right::read()));
    }

    #[test]
    fn rule_grant_success() {
        let mut graph = Graph::new();
        let root = graph.new_object();
        let root_a = graph.create(&root);
        let root_b = graph.create(&root);
        let root_c = graph.create(&root);
        let _root_a_a = graph.create(&root_a);
        let root_c_a = graph.create(&root_c);
        let _root_c_a_a = graph.create(&root_c_a);

        assert!(!graph.have_operation(&root_a, &root_b, &Right::read()));
        assert!(!graph.have_operation(&root_b, &root_c, &Right::read()));
        assert!(!graph.have_operation(&root_c, &root_b, &Right::read()));
        assert!(!graph.have_operation(&root_b, &root_a, &Right::read()));

        graph.grant(&root_a, &root_b);
        graph.grant(&root_b, &root_c);
        graph.grant(&root_c, &root_b);
        graph.grant(&root_b, &root_a);

        assert!(graph.have_operation(&root_a, &root_b, &Right::read()));
        assert!(graph.have_operation(&root_b, &root_c, &Right::read()));
        assert!(graph.have_operation(&root_c, &root_b, &Right::read()));
        assert!(graph.have_operation(&root_b, &root_a, &Right::read()));
    }

    #[test]
    fn rule_grant_fail() {
        let mut graph = Graph::new();
        let root = graph.new_object();
        let root_a = graph.create(&root);
        let root_b = graph.create(&root);
        let root_c = graph.create(&root);
        let root_a_a = graph.create(&root_a);
        let root_c_a = graph.create(&root_c);
        let _root_c_a_a = graph.create(&root_c_a);

        assert!(!graph.have_operation(&root_a_a, &root_b, &Right::read()));
        assert!(!graph.have_operation(&root_c, &root_a_a, &Right::read()));

        graph.grant(&root_a_a, &root_b);
        graph.grant(&root_c, &root_a_a);

        assert!(!graph.have_operation(&root_a_a, &root_b, &Right::read()));
        assert!(!graph.have_operation(&root_c, &root_a_a, &Right::read()));
    }

    #[test]
    fn can_operation_success() {
        let mut graph = Graph::new();
        let root = graph.new_object();
        let root_a = graph.create(&root);
        let root_b = graph.create(&root);
        let root_c = graph.create(&root);
        let root_a_a = graph.create(&root_a);
        let root_c_a = graph.create(&root_c);
        let root_c_a_a = graph.create(&root_c_a);

        assert!(graph.can_operation(&root_a_a, &root_c_a_a));
        assert!(graph.can_operation(&root_b, &root_c_a));
    }

    #[test]
    fn can_operation_fail() {
        let mut graph = Graph::new();
        let root = graph.new_object();
        let root_a = graph.create(&root);
        let root_b = graph.create(&root);
        let root_c = graph.create(&root);
        let root_a_a = graph.create(&root_a);
        let root_c_a = graph.create(&root_c);
        let root_c_a_a = graph.create(&root_c_a);

        let d = graph.new_object();
        let d_a = graph.new_object();

        assert!(!graph.can_operation(&root_c, &d));
        assert!(!graph.can_operation(&root_c_a_a, &d_a));
    }
}
