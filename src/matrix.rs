/// Represents an Adjascency Matrix for the [Graph]
#[derive(Debug)]
pub struct Matrix<'m> {
    matrix: Vec<Vec<Option<&'m str>>>
}

impl<'m> Matrix<'m> {

    pub fn new() -> Matrix<'m> {
        Self {
            matrix: vec![vec![]],
        }
    }

    pub fn add(&mut self) {
        self.matrix.push(vec![]);
    }
}