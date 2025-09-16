use sim::graph::Graph;

#[test]
fn can_add_edge_and_query_degree() {
    let mut g = Graph::new();
    g.add_edge(1usize, 2usize, 1usize);
    assert_eq!(g.degree(1usize), 1);
}
