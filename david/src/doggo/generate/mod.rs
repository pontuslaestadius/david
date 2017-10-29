extern crate image;

use std::fs::File;
use std::path::Path;

use std::io::prelude::*;
use self::image::{Rgba, ImageBuffer};

use super::Field;

pub fn generate(vec: Vec<Field>) {

    // Generates an new image.
    let mut dyn = image::DynamicImage::new_rgba8(30, 20);
    // Converts that image to be an imagebuffer with RGBA.
    let mut dyn_rgba = dyn.to_rgba();

    let primary: Rgba<u8> = match vec.get(0).unwrap() {
        &Field::Color(value) => value,
        _ => {println!("none"); Rgba {data: [0,0,0,255]}},
    };


    let offset = |a , b| {
        if a > 255-b {
            a-b
        } else if a+b < b {
            a+b
        } else {
            a+b
        }
    };

    let dif = 50;
    let secondary = Rgba {data: [offset(primary.data[0], dif), offset(primary.data[1], dif), offset(primary.data[2], dif), primary.data[3]]};

    tail(primary, secondary, 24, 7, &mut dyn_rgba);
    front_leg(primary, secondary, 7, 13, &mut dyn_rgba, 5); //background
    back_leg(primary, secondary, 19, 13, &mut dyn_rgba, 5); // background

    body(primary, secondary, 7, 8, &mut dyn_rgba);
    front_leg(primary, secondary, 10, 14, &mut dyn_rgba, 5);
    back_leg(primary, secondary, 22, 14, &mut dyn_rgba, 5);

    head(primary, secondary, 0,0,&mut dyn_rgba);

    // Save the image to local storage.
    let _ = dyn_rgba.save(&Path::new("test.png"));
}

// Generates a back leg of the dog at the given position.
pub fn back_leg(primary: Rgba<u8>, secondary: Rgba<u8>, x: u32, y: u32, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, height: u32) {
    // Adds leg
    for i in 0..height {
        image.put_pixel(x +i/2, y +i, primary);
        image.put_pixel(x +i/2 +1, y +i, secondary);
    }
}

// Generates a front leg of the dog at the given position.
pub fn front_leg(primary: Rgba<u8>, secondary: Rgba<u8>, x: u32, y: u32, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, height: u32) {

    // Adds leg
    for i in 0..height {
        image.put_pixel(x, y +i, primary);
        image.put_pixel(x+1, y +i, secondary);
    }
}

// Generates the tail of the dog at the given position.
pub fn tail(primary: Rgba<u8>, secondary: Rgba<u8>, x: u32, y: u32, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {

    let tail_size = 4;

    // Adds tail
    for i in 0..tail_size {
        image.put_pixel(x +i, y -i/2, primary);
        image.put_pixel(x +i +1, y -i/2, secondary);
    }
}

// Generates the body of the dog at the given position.
pub fn body(primary: Rgba<u8>, secondary: Rgba<u8>, x: u32, y: u32, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {

    let body_size = [7, 17];
    let body_size_front = [7, 14];
    let body_size_middle = [7, 17];
    let body_size_back = [7, 13];

    // Adds body
    for i in 0..body_size[1] {
        for j in 0..body_size[0] {
            if (j == body_size[0]-1 && i > 1 && i != body_size[1]-1) {
                image.put_pixel(x +i, y +j, secondary);
            } else {
                image.put_pixel(x +i, y +j, primary);
            }

        }
    }

}

// Generates the head of the dog at the given position.
pub fn head(primary: Rgba<u8>, secondary: Rgba<u8>, x: u32, y: u32, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {


    let data: [u8; 4] = [0, 0, 0, 255];
    let black = Rgba {data};

    // how many pixels is the head.
    let nose_size = 4;
    let head_size: u32 = 10;
    let ear_size = 2;

    // Adds head
    for i in 0..head_size {
        for j in 0..head_size {
            // Put the pixel on the imagebuffer.
            if (i == head_size-1 && j == head_size-1) ||
                (i == head_size-1 && j == 0) ||
                (i == 0 && j == head_size-1) {
                continue;
            }

            if i == head_size-1 || j == head_size-1 {
                image.put_pixel(x +i +nose_size/2, y +j + ear_size, secondary);
            } else {
                image.put_pixel(x +i +nose_size/2, y +j + ear_size, primary);
            }

        }
    }

    // Adds ears
    for h in 0..2 {
        for i in 0..ear_size {
            for j in 0..ear_size {
                if i == ear_size-1 && j == 0 {continue;}
                if j == ear_size-1 && i == 0 {
                    image.put_pixel(x+nose_size/2 + h*(head_size-ear_size*2) +i, y +j, secondary);
                } else {
                    image.put_pixel(x+nose_size/2 + h*(head_size-ear_size*2) +i, y +j, primary);
                }
            }
        }
    }

    // Adds nose.
    let nose_start_y = (head_size/2) as u32;
    for i in 0..(nose_size as f32 *1.5) as u32 {
        for j in 0..nose_size {
            if j == nose_size && i == 0 {continue;}
            // Put the pixel on the imagebuffer.

            let color = match j == 0 {
                true => secondary,
                false => primary,
            };

            image.put_pixel(x +i, y +j + nose_start_y, color);
        }
    }

    for i in 0..nose_size/2 {
        for j in 0..(nose_size+1)/2 {
            // Put the pixel on the imagebuffer.
            image.put_pixel(x +i, y +j + nose_start_y, black);
        }
    }

    // Adds eyes
    image.put_pixel(x +head_size/3, y +nose_start_y-1, black);
    image.put_pixel(x +head_size*2/3, y +nose_start_y-1, black);

    // Adds mouth
    for i in 0..head_size/3 {
        image.put_pixel(x +nose_size/2 +i, y +nose_start_y+nose_size, black);
    }
}
