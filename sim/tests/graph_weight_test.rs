use sim::graph::Graph;

#[test]
fn weight_is_stored_and_is_undirected() {
    let mut g = Graph::new();
    g.add_edge(1usize, 2usize, 7usize);
    assert_eq!(g.weight(1usize, 2usize), Some(&7usize));
    assert_eq!(g.weight(2usize, 1usize), Some(&7usize));
}
