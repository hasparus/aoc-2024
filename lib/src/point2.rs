#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Point2 {
    pub row: usize,
    pub col: usize,
}

impl Point2 {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

impl From<Point2> for (usize, usize) {
    fn from(point: Point2) -> Self {
        (point.row, point.col)
    }
}

impl From<(usize, usize)> for Point2 {
    fn from(point: (usize, usize)) -> Self {
        Self {
            row: point.0,
            col: point.1,
        }
    }
}

impl std::ops::Add for Point2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

impl std::ops::Sub for Point2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            row: self.row - other.row,
            col: self.col - other.col,
        }
    }
}

impl std::ops::Add<(isize, isize)> for Point2 {
    type Output = Self;

    fn add(self, (row, col): (isize, isize)) -> Self {
        Self {
            row: (self.row as isize + row) as usize,
            col: (self.col as isize + col) as usize,
        }
    }
}

impl std::ops::Add<&(isize, isize)> for Point2 {
    type Output = Self;

    fn add(self, (row, col): &(isize, isize)) -> Self {
        Self {
            row: (self.row as isize + row) as usize,
            col: (self.col as isize + col) as usize,
        }
    }
}

impl std::ops::Sub<(isize, isize)> for Point2 {
    type Output = Self;

    fn sub(self, (row, col): (isize, isize)) -> Self {
        Self {
            row: (self.row as isize - row) as usize,
            col: (self.col as isize - col) as usize,
        }
    }
}

impl std::ops::Mul<usize> for Point2 {
    type Output = Self;

    fn mul(self, scalar: usize) -> Self {
        Self {
            row: self.row * scalar,
            col: self.col * scalar,
        }
    }
}

impl std::ops::Div<usize> for Point2 {
    type Output = Self;

    fn div(self, scalar: usize) -> Self {
        Self {
            row: self.row / scalar,
            col: self.col / scalar,
        }
    }
}
