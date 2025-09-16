#[test]
fn graph_is_generic_over_payload() {
    use sim::graph::Graph;

    // Scalar payload
    let mut gs: Graph<u32> = Graph::new();
    gs.add_edge(1usize, 2usize, 7u32);
    assert_eq!(gs.weight(1usize, 2usize), Some(&7u32));

    // Vector payload
    let mut gv: Graph<Vec<f32>> = Graph::new();
    gv.add_edge(3usize, 4usize, vec![1.0, 2.0]);
    assert_eq!(gv.weight(3usize, 4usize).unwrap(), &vec![1.0, 2.0]);
}
