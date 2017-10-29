extern crate image;

pub mod decode;
pub mod generate;

pub enum Field {
    None,
    Color(image::Rgba<u8>),
    Mood(Mood),
}

pub enum Mood {
    Happy,
    Sad,
}

pub struct Default {
    color: Field::Color(),
    mood: Mood,
}

impl Default {
    pub fn new() -> Default {
        Default {
            color: image::Rgba {data: [130, 130, 130, 255]},
            mood: Mood::Happy
        }
    }
}