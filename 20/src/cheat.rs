use aoc_2024_lib::point2::Point2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cheat {
    pub start: Point2,
    pub end: Point2,
    pub length_saved: usize,
}
