use std::collections::HashMap;

use crate::matrix::Matrix;

#[derive(Debug)]
struct Graph<'g> {
    vertices: HashMap<&'g str, usize>,
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
        let from_index = self.add_vertex_if_not_exists(from);
        let to_index = self.add_vertex_if_not_exists(to);
        self.adj_matrix.set_option(from_index, to_index, Some(name))
    }


    /// Adds a vertex in the graph.
    /// If the vertex already exists, function does nothing.
    /// 
    /// Returns the index position of the vertex
    fn add_vertex_if_not_exists(&mut self, vertex: &'g str) -> usize {
        let index = self.vertices.get(vertex);
        match index {
            Some(index) => { *index },
            None => { 
                let index = self.vertices.len().try_into().unwrap();
                self.vertices.insert(vertex, index);
                self.adj_matrix.add_row();
                index
            }
        }
    }

}

#[cfg(test)]
#[test]
fn should_graph() {
    let mut graph = Graph::new();
    graph.add_edge("TURN_ON", "OFF", "ON");
    graph.add_edge("TURN_OFF", "ON", "OFF");
    graph.add_edge("BREAK", "ON","BROKEN");
    println!("{:?}", graph);
}