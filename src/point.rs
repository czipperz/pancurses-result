/// A two-dimensional point
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub y: i32,
    pub x: i32,
}

impl From<(i32, i32)> for Point {
    fn from(v: (i32, i32)) -> Self {
        Point { y: v.0, x: v.1 }
    }
}

impl From<Point> for (i32, i32) {
    fn from(p: Point) -> (i32, i32) {
        (p.y, p.x)
    }
}

/// A two-dimensional dimension
pub struct Dimension {
    pub rows: i32,
    pub columns: i32,
}

impl From<(i32, i32)> for Dimension {
    fn from(v: (i32, i32)) -> Self {
        Dimension {
            rows: v.0,
            columns: v.1,
        }
    }
}

impl From<Dimension> for (i32, i32) {
    fn from(p: Dimension) -> (i32, i32) {
        (p.rows, p.columns)
    }
}
