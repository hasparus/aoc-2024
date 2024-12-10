use std::convert::TryInto;
use std::fmt::Display;
use std::ops::Index;

#[derive(Debug)]
pub struct Grid<T> {
    data: Vec<Vec<T>>,
    width: usize,
    height: usize,
    default: T,
}

impl<T, I> Index<(I, I)> for Grid<T>
where
    I: TryInto<usize>,
    I::Error: std::fmt::Debug,
{
    type Output = T;

    fn index(&self, (row, col): (I, I)) -> &Self::Output {
        let row: usize = row.try_into().expect("row index conversion failed");
        let col: usize = col.try_into().expect("column index conversion failed");
        &self.data[row][col]
    }
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

    pub fn in_bounds<I>(&self, row: I, col: I) -> bool
    where
        I: TryInto<isize> + std::cmp::PartialOrd + From<u8>,
        I::Error: std::fmt::Debug,
    {
        row >= 0.into()
            && col >= 0.into()
            && row.try_into().expect("row conversion failed") < self.height as isize
            && col.try_into().expect("col conversion failed") < self.width as isize
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
