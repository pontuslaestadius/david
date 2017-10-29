extern crate image;

pub mod decode;
pub mod generate;

pub enum Field {
    None,
    Color(image::Rgba<u8>),
}
