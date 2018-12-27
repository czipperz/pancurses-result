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
