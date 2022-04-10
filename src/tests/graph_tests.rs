use crate::graph::Graph;

#[test]
fn should_init_graph() {
    let graph = Graph::new();
    assert_eq!(0, graph.vertices.len());
}

#[test]
fn should_graph() {
    let mut graph = Graph::new();
    graph.add_edge("TURN_ON".to_string(), "OFF".to_string(), "ON".to_string());
    graph.add_edge("TURN_OFF".to_string(), "ON".to_string(), "OFF".to_string());
    graph.add_edge("BREAK".to_string(), "ON".to_string(),"BROKEN".to_string());
}