/// Represents an Adjascency Matrix for the [Graph]
#[derive(Debug)]
pub struct Matrix<'m> {
    matrix: Vec<Vec<Option<&'m str>>>
}

impl<'m> Matrix<'m> {

    pub fn new() -> Matrix<'m> {
        Self {
            matrix: vec![],
        }
    }

    pub fn add_row(&mut self) {
        self.matrix.iter_mut().for_each(|row| row.push(None));
        self.matrix.push(vec![None; self.matrix.len() + 1]);
    }

    pub fn set_option(&mut self, x: usize, y: usize, option: Option<&'m str>) {
        self.matrix[x][y] = option;
    }
}