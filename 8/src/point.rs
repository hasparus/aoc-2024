#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub row: isize,
    pub col: isize,
}

impl Point {
    pub fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            row: self.row - other.row,
            col: self.col - other.col,
        }
    }
}

impl std::ops::Mul<isize> for Point {
    type Output = Self;

    fn mul(self, scalar: isize) -> Self {
        Self {
            row: self.row * scalar,
            col: self.col * scalar,
        }
    }
}

impl std::ops::Div<isize> for Point {
    type Output = Self;

    fn div(self, scalar: isize) -> Self {
        Self {
            row: self.row / scalar,
            col: self.col / scalar,
        }
    }
}
