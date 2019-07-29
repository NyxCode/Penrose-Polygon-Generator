extern crate impossible_polygon;

use std::fs;
use impossible_polygon::generate_penrose_polygon;

fn main() {
    let colors = vec!["#FF0000".to_string(),
                      "#00FF00".to_string(),
                      "#0000FF".to_string()];
    let svg = generate_penrose_polygon(3, false, 0., 0.5, &colors);
    fs::write("image.svg", svg).expect("Unable to write file");
}