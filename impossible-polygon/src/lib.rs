mod types;
mod utils;

use utils::*;
use types::*;
use std::f64::consts::PI;
use svg::Document;

pub fn generate_penrose_polygon(n: u32,
                                debug: bool,
                                thickness_modifier: f64,
                                perspective_modifier: f64,
                                color_palette: &Vec<String>) -> String {
    let outer_side_len = get_outer_side_len(n, thickness_modifier, perspective_modifier);

    render(n, debug, &color_palette, outer_side_len, perspective_modifier).unwrap().to_string()
}


fn render(n: u32,
          debug: bool,
          colors: &Vec<String>,
          outer_side_len: f64,
          perspective_modifier: f64) -> Result<Document, ()> {
    let mut document = Document::new();

    let alpha = inner_angle_of_regular_polygon(n);

    let inner_polygon = gen_polygon(n, 10.0, 0.0);
    if debug {
        document = draw_lines(document, &inner_polygon, "red");
    }

    let outer_polygon = gen_polygon(n, outer_side_len, PI / n as f64);
    if debug {
        document = draw_lines(document, &outer_polygon, "green");
    }

    let mut intersections = get_intersections(&outer_polygon, &inner_polygon);
    if intersections.len() != 2 * n as usize {
        return Err(());
    }

    if n % 2 == 0 {
        rotate_vec(&mut intersections);
    }

    let constructed_polygon = connect_points(&intersections);
    if debug { document = draw_lines(document, &constructed_polygon, "black"); }


    let ls: Vec<_> = intersections
        .iter()
        .enumerate()
        .step_by(2)
        .map(|(index, _)| {
            let next_index = (index + 3) % intersections.len();
            line(intersections[index], intersections[next_index])
        })
        .collect();

    if debug { document = draw_lines(document, &ls, "yellow"); }

    let b = {
        let a_b_dist = (*&intersections[0] - *&intersections[1]).magnitude();
        f64::sin(PI - alpha) / f64::sin(alpha) * f64::sin((PI - alpha) / 2.0) * a_b_dist
    };

    let shifted_ls = {
        // 'intersections' goes from 0 to 2n-1
        let mut shifted_intersections = intersections.clone();
        rotate_vec(&mut shifted_intersections);

        let mut lines = vec![];
        for chunk in shifted_intersections.chunks(2) {
            let i: Point = chunk[0];
            let j: Point = chunk[1];
            let i_to_j = j - i;
            let normal = point(-i_to_j.y, i_to_j.x);
            let unit_normal = normal / normal.magnitude();
            let point = i + i_to_j / 2.0 - unit_normal * b * (1. + perspective_modifier);

            let line = line(point - i_to_j * 10.0, point + i_to_j * 10.0);
            if debug { document = document.add(line.to_svg("blue")); }
            lines.push(line);
        }
        lines
    };

    for e in 0..(n as usize) {
        let points = vec![*&intersections[2 * e],
                          *&intersections[2 * e + 1],
                          *intersections.get_wrap(2 * e + 2),
                          line(*intersections.get_wrap(2 * e + 2),
                               *intersections.get_wrap(2 * e + 5))
                              .intersect(*shifted_ls.get_wrap(e + 3)).ok_or(())?,
                          shifted_ls.get_wrap(e + 3)
                              .intersect(*shifted_ls.get_wrap(e + 2)).ok_or(())?,
                          shifted_ls.get_wrap(e + 3)
                              .intersect(*shifted_ls.get_wrap(e + 2)).ok_or(())?,
                          line(*intersections.get_wrap(2 * e), *intersections.get_wrap(2 * e + 3))
                              .intersect(*shifted_ls.get_wrap(e + 2)).ok_or(())?,
        ];
        let color = &colors[e % colors.len()];

        if debug {
            document = draw_lines(document, &connect_points(&points), "black");
        } else {
            document = draw_polygon(document, &points, color);
        }
    }

    Ok(set_viewbox(document, &constructed_polygon))
}


fn gen_polygon(n: u32, side_len: f64, rotation: f64) -> Vec<Line> {
    let radius = calculate_regular_polygon_radius(n, side_len);
    let edges = generate_regular_polygon(n, radius, rotation);
    let lines = connect_points(&edges);

    lines
}

/// brute-forces the side length of the outer polygon used to cut the corners of the inner polygon.
/// this function first calculates the possible range of side lengths and then returns a concrete
/// value through the value of `modifier`
/// `0` will result in the lowest possible side length, `1` in the largest possible side length.
fn get_outer_side_len(n: u32, thickness_modifier: f64, perspective_modifier: f64) -> f64 {
    assert!(thickness_modifier >= 0.0 && thickness_modifier <= 1.0);

    // we brute-force the minimum and maximum dimensions of the outer polygon
    // used to cut the edges of the inner polygon.
    let (min, max) = {
        let mut min = 1.0;
        // we start with a polygon which perfectly contains the inner polygon.
        // even though this _should_ be the maximum side length,
        // floating point precision errors may still cause problems
        let mut max = {
            let alpha = inner_angle_of_regular_polygon(n);
            (2.0 * 10.0 * ((PI - alpha) / 2.0).sin()) / alpha.sin()
        };

        while render(n, false, &vec!["red".to_string()], min, perspective_modifier).is_err() {
            min += 0.1;
        }

        while render(n, false, &vec!["red".to_string()], max, perspective_modifier).is_err() {
            max -= 0.1;
        }

        (min, max)
    };

    // Even though the algorithm will work with all values between `min` and `max`,
    // some values will cause the resulting image of polygons with n=3/4/5 to look bad.
    // These values are determined by trial and error.
    // For polygons with n > 5, this is not necessary.
    let recommended_min = match n {
        3 => 0.5,
        4 => 0.48,
        5 => 0.26,
        _ => 0.0
    };

    min + (max - min) * (recommended_min + (1.0 - recommended_min) * thickness_modifier)
}
