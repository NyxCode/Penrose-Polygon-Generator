use std::ops::{Sub, Add, AddAssign, Div, Mul};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Line {
    pub a: Point,
    pub b: Point,
}

pub fn point(x: f64, y: f64) -> Point {
    Point { x, y }
}

pub fn line(a: Point, b: Point) -> Line {
    Line { a, b }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        point(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        point(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Div<f64> for Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Self::Output {
        Point { x: self.x / rhs, y: self.y / rhs }
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        Point { x: self.x * rhs, y: self.y * rhs }
    }
}

impl Point {
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


impl Line {
    pub fn intersect(self, other: Self) -> Option<Point> {
        let a1 = self.b.y - self.a.y;
        let b1 = self.a.x - self.b.x;
        let c1 = a1 * self.a.x + b1 * self.a.y;

        let a2 = other.b.y - other.a.y;
        let b2 = other.a.x - other.b.x;
        let c2 = a2 * other.a.x + b2 * other.a.y;

        let delta = a1 * b2 - a2 * b1;

        if delta == 0.0 {
            return None;
        }

        let intersection = point(
            (b2 * c1 - b1 * c2) / delta,
            (a1 * c2 - a2 * c1) / delta,
        );

        if self.contains_point(intersection) {
            Some(intersection)
        } else {
            None
        }
    }

    fn contains_point(&self, point: Point) -> bool {
        // point = self.0 + j(self.1 - self.0)
        let a_to_b = (self.b - self.a).magnitude();

        let a_to_point = (point - self.a).magnitude();
        let b_to_point = (point - self.b).magnitude();

        a_to_b >= a_to_point && a_to_b >= b_to_point
    }

    pub fn to_svg<S: Into<String>>(&self, color: S) -> svg::node::element::Line {
        let Line {
            a: Point { x: x1, y: y1 },
            b: Point { x: x2, y: y2 }
        } = self;

        svg::node::element::Line::new()
            .set("x1", x1.to_string())
            .set("y1", y1.to_string())
            .set("x2", x2.to_string())
            .set("y2", y2.to_string())
            .set("stroke", color.into())
            .set("stroke-width", "0.3")
            .set("stroke-linecap", "round")

    }
}
