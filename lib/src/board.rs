use crate::point2::Point2;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board<T>(pub Vec<Vec<T>>);

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

impl<T> std::ops::IndexMut<Point2> for Board<T> {
    fn index_mut(&mut self, index: Point2) -> &mut Self::Output {
        &mut self.0[index.row][index.col]
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
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| {
                        c.to_string()
                            .parse::<T>()
                            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
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
