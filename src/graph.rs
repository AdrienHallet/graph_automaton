use std::collections::HashMap;

use crate::matrix::Matrix;

#[derive(Debug)]
struct Graph<'g> {
    vertices: HashMap<&'g str, u8>,
    adj_matrix: Matrix<'g>,
}

impl<'g> Graph<'g> {

    pub fn new() -> Graph<'g> {
        Self {
            vertices: HashMap::new(),
            adj_matrix: Matrix::new(),
        }
    }

    pub fn add_edge(&mut self, name: &'g str, from: &'g str, to: &'g str) {
        self.add_vertex_if_not_exists(from);
        self.add_vertex_if_not_exists(to);
        
    }


    /// Adds a vertex in the graph.
    /// If the vertex already exists, function does nothing.
    fn add_vertex_if_not_exists(&mut self, vertex: &'g str) {
        if !self.vertices.contains_key(vertex) {
            self.vertices.insert(vertex, self.vertices.len().try_into().unwrap());
            self.adj_matrix.add()
        }
    }

}

#[cfg(test)]
#[test]
fn should_graph() {
    let mut graph = Graph::new();
    graph.add_edge("Event", "Input", "Output");
    println!("{:?}", graph);
}