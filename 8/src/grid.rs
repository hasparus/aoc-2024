use std::fmt::Display;

#[derive(Debug)]
pub struct Grid<T> {
    data: Vec<Vec<T>>,
    width: usize,
    height: usize,
    default: T,
}

impl<T: Clone + Display> Grid<T> {
    pub fn new(data: Vec<Vec<T>>, default: T) -> Self {
        let height = data.len();
        let width = if height > 0 { data[0].len() } else { 0 };
        Self {
            data,
            width,
            height,
            default,
        }
    }

    pub fn get(&self, row: isize, col: isize) -> &T {
        if !self.in_bounds(row, col) {
            &self.default
        } else {
            &self.data[row as usize][col as usize]
        }
    }

    pub fn in_bounds(&self, row: isize, col: isize) -> bool {
        row >= 0 && col >= 0 && row < self.height as isize && col < self.width as isize
    }

    pub fn cells(&self) -> impl Iterator<Item = (isize, isize, &T)> {
        (0..self.height).flat_map(move |row| {
            (0..self.width).map(move |col| (row as isize, col as isize, &self.data[row][col]))
        })
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }
}
