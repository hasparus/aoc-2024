#[derive(Debug, Clone, Copy)]
pub struct Point2 {
    pub row: usize,
    pub col: usize,
}

impl Point2 {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}
