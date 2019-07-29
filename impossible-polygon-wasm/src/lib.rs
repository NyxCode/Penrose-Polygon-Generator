use wasm_bindgen::prelude::*;
use impossible_polygon::generate_penrose_polygon as gen;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
pub fn generate_penrose_polygon(n: u32,
                                debug: bool,
                                thickness_modifier: f64,
                                perspective_modifier: f64,
                                color_palette: &JsValue) -> String {

    let color_palette = color_palette.into_serde::<StringVec>().unwrap().0;
    gen(n, debug, thickness_modifier, perspective_modifier, &color_palette)
}

#[derive(Serialize, Deserialize)]
pub struct StringVec(Vec<String>);
