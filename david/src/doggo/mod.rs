extern crate image;

pub mod decode;
pub mod generate;

pub enum Field {
    None,
    Colour(Colour),
    Mood(Mood),
}

pub enum Mood {
    Happy,
    Sad,
}

pub struct Colour {
    pub colour: image::Rgba<u8>,
}

pub struct Default {
    colour: Colour,
    mood: Mood,
}

impl Default {
    pub fn new() -> Default {
        Default {
            colour: Colour {colour: image::Rgba {data: [130, 130, 130, 255]}},
            mood: Mood::Happy
        }
    }
}