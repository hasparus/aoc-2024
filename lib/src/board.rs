use crate::point2::Point2;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board<T>(pub Vec<Vec<T>>);

impl<T> Board<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        Self(data)
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Board<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            writeln!(
                f,
                "{}",
                row.iter().map(|t| t.to_string()).collect::<String>()
            )?;
        }
        Ok(())
    }
}

impl<T> std::ops::Index<(u8, u8)> for Board<T> {
    type Output = T;

    fn index(&self, index: (u8, u8)) -> &Self::Output {
        &self.0[index.0 as usize][index.1 as usize]
    }
}

impl<T> std::ops::IndexMut<(u8, u8)> for Board<T> {
    fn index_mut(&mut self, index: (u8, u8)) -> &mut Self::Output {
        &mut self.0[index.0 as usize][index.1 as usize]
    }
}

impl<T> std::ops::Index<usize> for Board<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> std::ops::IndexMut<usize> for Board<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T> std::ops::Index<Point2> for Board<T> {
    type Output = T;

    fn index(&self, index: Point2) -> &Self::Output {
        &self.0[index.row][index.col]
    }
}

impl<T> std::ops::Index<(isize, isize)> for Board<T> {
    type Output = T;

    fn index(&self, index: (isize, isize)) -> &Self::Output {
        &self.0[index.0 as usize][index.1 as usize]
    }
}

impl<T> std::ops::IndexMut<(isize, isize)> for Board<T> {
    fn index_mut(&mut self, index: (isize, isize)) -> &mut Self::Output {
        &mut self.0[index.0 as usize][index.1 as usize]
    }
}

impl<T> std::ops::Index<&Point2> for Board<T> {
    type Output = T;

    fn index(&self, index: &Point2) -> &Self::Output {
        &self.0[index.row][index.col]
    }
}

impl<T> std::ops::IndexMut<&Point2> for Board<T> {
    fn index_mut(&mut self, index: &Point2) -> &mut Self::Output {
        &mut self.0[index.row][index.col]
    }
}

impl<T> Board<T> {
    pub fn iter(&self) -> impl Iterator<Item = &Vec<T>> {
        self.0.iter()
    }
}

impl<T: std::str::FromStr> Board<T>
where
    T::Err: std::error::Error + 'static,
{
    pub fn from_lines(lines: &[&str]) -> Result<Self, Box<dyn std::error::Error>> {
        let tokens = lines
            .iter()
            .filter(|line| !line.trim().is_empty())
            .enumerate()
            .map(|(line_number, line)| {
                line.trim()
                    .chars()
                    .enumerate()
                    .map(|(column, c)| {
                        c.to_string().parse::<T>().map_err(|e| {
                            format!("failed to parse token `{c}` at {line_number}:{column}: {e}")
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Board(tokens))
    }
}

impl<T: std::str::FromStr> std::str::FromStr for Board<T>
where
    T::Err: std::error::Error + 'static,
{
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Board::from_lines(&s.lines().collect::<Vec<_>>())
    }
}

impl<T: PartialEq + std::fmt::Debug> Board<T> {
    pub fn find(&self, searched: &T) -> Point2 {
        for (i, row) in self.iter().enumerate() {
            for (j, token) in row.iter().enumerate() {
                if token == searched {
                    return Point2::new(i, j);
                }
            }
        }

        panic!("{:?} not found", searched);
    }
}

impl<T> Board<T> {
    pub fn in_bounds(&self, point: (isize, isize)) -> bool {
        let rows = self.0.len();
        let cols = self.0[0].len();

        point.0 >= 0 && point.0 < rows as isize && point.1 >= 0 && point.1 < cols as isize
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn width(&self) -> usize {
        self.0[0].len()
    }
}
