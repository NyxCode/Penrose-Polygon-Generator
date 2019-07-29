
use crate::types::*;
use std::f64::consts::PI;
use svg::Document;
use svg::node::element::Polygon;

/// `get_wrap` gets an element from a `Vec` at a specific index.
/// Instead of causing a 'index out of bounds' panic, the index is 'wrapped around',
/// so `vec![1, 2, 3].get_wrap(3)` returns `1`
pub trait GetWrapping<T> {
    fn get_wrap(&self, index: usize) -> &T;
}

impl <T> GetWrapping<T> for Vec<T> {
    fn get_wrap(&self, index: usize) -> &T {
        assert!(!self.is_empty());
        &self[index % self.len()]
    }
}

/// Generates a regular polygon with n edges rotated by 'phi' radians.
/// All points are 'radius' away from the origin.
pub fn generate_regular_polygon(n: u32, radius: f64, phi: f64) -> Vec<Point> {
    let delta_angle = 2.0 * PI / n as f64;
    (0..n)
        .map(|n| n as f64 * delta_angle + phi)
        .map(|a| point(a.sin() * radius, a.cos() * radius))
        .collect()
}

/// Returns the inner angle of a regular polygon with n edges.
///
/// example:
///     `inner_angle_of_regular_polygon(3)` -> `rad(60°)`
///     `inner_angle_of_regular_polygon(4)` -> `rad(90°)`
pub fn inner_angle_of_regular_polygon(n: u32) -> f64 {
    f64::from(n - 2) * PI / f64::from(n)
}

/// Calculates all intersections between two set of lines.
/// Intersections between lines in one set are not returned.
/// The calculated intersections are sorted by their angle on the unit circle.
pub fn get_intersections(poly1: &Vec<Line>, poly2: &Vec<Line>) -> Vec<Point> {
    let mut points = vec![];

    for line1 in poly1 {
        for line2 in poly2 {
            if let Some(intersection) = line1.intersect(*line2) {
                points.push(intersection);
            }
        }
    }

    let mut average = point(0.0, 0.0);
    for p in &points {
        average = point(average.x + p.x,
                        average.y + p.y);
    }
    average = point(average.x / points.len() as f64,
                    average.y / points.len() as f64);

    let mut p: Vec<_> = points.iter()
        .map(|point| (point, *point - average))
        .map(|(point, avg)| (point, f64::atan2(avg.x, avg.y)))
        .collect();
    p.sort_by(|(_, atan1), (_, atan2)| (*atan1).partial_cmp(atan2).unwrap());


    p.iter().map(|(point, _)| **point).collect()
}

/// Calculate the radius of a polygon with the given side length and edges.
///
/// example:
///     a circle around a square with side length 1 has a radius of sqrt(2)
///     `calculate_regular_polygon_radius(4, 1)` -> `sqrt(2)`
pub fn calculate_regular_polygon_radius(n: u32, side_len: f64) -> f64 {
    let alpha = inner_angle_of_regular_polygon(n);
    (side_len * (alpha / 2.0).sin()) / (PI - alpha).sin()
}

/// Connects points, resulting in a `Vec` of `Line`s.
///
/// `points[0]`     is connected to `points[1]`
/// `points[1]`     is connected to `points[2]`
/// `points.last()` is connected to `points[0]`
pub fn connect_points(points: &Vec<Point>) -> Vec<Line> {
    assert!(points.len() >= 3);

    let mut lines = Vec::with_capacity(points.len());
    let mut iterator = points.iter().peekable();
    let first_point = iterator.next().unwrap();

    let mut last_point = first_point;

    while iterator.peek().is_some() {
        let current_point = iterator.next().unwrap();
        let line = line(*last_point, *current_point);
        lines.push(line);
        last_point = current_point;
    }

    let last_line = line(*last_point, *first_point);

    lines.push(last_line);

    lines
}

/// Moves every entry in `vec` one to the right while moving the last entry to the start
pub fn rotate_vec<E>(vec: &mut Vec<E>) {
    let last_element = vec.pop().expect("vec is empty");
    vec.insert(0, last_element);
}

/// Draws lines of the given color into the document
pub fn draw_lines<S: Into<String> + Copy>(document: Document, lines: &Vec<Line>, color: S) -> Document {
    let mut doc = document;
    for line in lines {
        doc = doc.add(line.to_svg(color))
    }

    doc
}

/// Draws a polygon of the given color described by the given points to the document
pub fn draw_polygon<S: Into<String>>(document: Document, points: &Vec<Point>, color: S) -> Document {
    let points_str = points
        .iter()
        .map(|point| format!("{},{}", point.x, point.y))
        .collect::<Vec<_>>()
        .join(" ");

    let polygon = Polygon::new()
        .set("points", points_str)
        .set("fill", color.into());

    document.add(polygon)
}

/// sets the viewbox of the given document to completely display all given lines
pub fn set_viewbox(document: Document, lines: &Vec<Line>) -> Document {
    let cords = lines.iter().flat_map(|line| vec![line.a, line.b]);
    let min_x = cords.clone().map(|cord| cord.x).fold(1. / 0., f64::min) * 1.1;
    let min_y = cords.clone().map(|cord| cord.y).fold(1. / 0., f64::min) * 1.1;
    let max_x = cords.clone().map(|cord| cord.x).fold(0. / 0., f64::max) * 1.1;
    let max_y = cords.clone().map(|cord| cord.y).fold(0. / 0., f64::max) * 1.1;
    document.set("viewBox", (min_x, min_y, max_x - min_x, max_y - min_y))
}