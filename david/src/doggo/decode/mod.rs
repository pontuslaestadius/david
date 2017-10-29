
extern crate image;

use super::Field;
use std::fs::File;
use std::io::prelude::*;

use self::image::Rgba;

// Decodes a file and returns a vector of all the fields.
pub fn decode(mut file: &File) -> Vec<Field> {
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);

    let mut vec = Vec::new();
    for line in string.split(";") {
        let field = str_to_field(line);
        vec.push(field)
    }
    vec
}

// Converts a given str to the appropriate Field enum. None if none apply.
pub fn str_to_field(str: &str) -> Field {
    let mut field = str.split(":");

    match field.next().unwrap() {
        "color" => Field::Color(decode_color(field.next().unwrap())),
        _ => Field::None,
    }
}

pub fn decode_color(str: &str) -> image::Rgba<u8> {
    // If it's a plain color or if it's RGBA
    let mut data: [u8; 4];
    if str.contains("[") { // RGBA
        // TODO
        data = [0,0,0,255];
    } else { // Plain
        match str {
            "black"     => data = [0, 0, 0, 255],
            "white"     => data = [255, 255, 255, 255],
            "red"       => data = [255, 0, 0, 255],
            "green"     => data = [0, 255, 0, 255],
            "blue"      => data = [0, 0, 255, 255],
            "brown"     => data = [100, 60, 0, 255],
            "pink"     => data = [255, 105, 180, 255],
            _           => data = [0, 0, 0, 255],
        }
    }

    println!("{:?}", data);
    Rgba {data}
}