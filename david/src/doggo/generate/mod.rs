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

    let darken_rgba = |a: Rgba<u8>| {
        let v = 30;
        Rgba {data: [darken(a.data[0], v), darken(a.data[1], v), darken(a.data[2], v), a.data[3]]}
    };

    let dif = 50;
    let secondary = Rgba {data: [offset(primary.data[0], dif), offset(primary.data[1], dif), offset(primary.data[2], dif), primary.data[3]]};

    front_leg(darken_rgba(primary), darken_rgba(secondary), 6, 13, &mut dyn_rgba, 5); //background
    back_leg(darken_rgba(primary), darken_rgba(secondary), 17, 13, &mut dyn_rgba, 5); // background

    let (tailx, taily) = body(primary, secondary, 5, 8, &mut dyn_rgba);
    front_leg(primary, secondary, 9, 14, &mut dyn_rgba, 5);
    back_leg(primary, secondary, 20, 14, &mut dyn_rgba, 5);
    tail(primary, secondary, tailx, taily, &mut dyn_rgba, Mood::Happy);

    let _ = head(primary, secondary, 0,0,&mut dyn_rgba);

    // Save the image to local storage.
    let _ = dyn_rgba.save(&Path::new("test.png"));
}

// Generates a back leg of the dog at the given position.
pub fn back_leg(primary: Rgba<u8>, secondary: Rgba<u8>, x: u32, y: u32, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, height: u32) {
    // Adds leg
    for i in 0..height {
        image.put_pixel(x +i/3, y +i, primary);
        image.put_pixel(x +i/3 +1, y +i, secondary);
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
pub fn tail(primary: Rgba<u8>, secondary: Rgba<u8>, x: u32, y: u32, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, mood: Mood) {
    let tail_size = 4;

    let offset: i32 =
    match mood {
        Mood::Happy => 1,
        Mood::Sad => -1,
        _ => 1,
    };

    for i in 0..tail_size {
        image.put_pixel(x +i, (y as i32 -((i) as i32*offset)) as u32, primary);
        image.put_pixel(x +i +1, (y as i32 -((i) as i32*offset)) as u32, secondary);
    }
}

// Generates the body of the dog at the given position.
pub fn body(primary: Rgba<u8>, secondary: Rgba<u8>, x: u32, y: u32, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) -> (u32, u32) {

    // width, height.
    let body_size_front = [5, 6];
    let body_size_middle = [6, 6];
    let body_size_back = [5, 6];

    // Adds body

    // Front
    for i in 0..body_size_front[0] {
        for j in 0..body_size_front[1] {
            if i == 0 && j == body_size_front[1]-1 {continue;}
            image.put_pixel(x +i, y +j, primary);
        }
    }

    // Middle
    for i in 0..body_size_middle[1] {
        for j in 0..body_size_middle[0] {
            if j == body_size_middle[0]-1 {
                image.put_pixel(x +i + body_size_front[0], y +j, secondary);

            } else {
                image.put_pixel(x +i + body_size_front[0], y +j, primary);
            }
        }
    }

    // Back
    for i in 0..body_size_back[0] {
        for j in 0..body_size_back[1] {
            if (i > body_size_back[0]-2 && j < 2) {continue;}

            image.put_pixel(x +i + body_size_front[0] + body_size_middle[0], y +j, primary);
        }
    }

    (x +body_size_front[0] +body_size_middle[0] +body_size_back[0] -1, y)
}

// Generates the head of the dog at the given position.
pub fn head(primary: Rgba<u8>, secondary: Rgba<u8>, x: u32, y: u32, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) -> (u32, u32) {

    let data: [u8; 4] = [0, 0, 0, 255];
    let black = Rgba {data};

    // how many pixels is the head.
    let nose_size = 3;
    let head_size: u32 = 8;
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

    (x + head_size/2, y + head_size/2)
}

// Reduces the value of a u8 with the provided b value.
fn darken(a: u8, b: u8) -> u8 {
    if a == 0 { // Nothing to lower
        a
    } else if a > 255-b { // If it can be lowered.
        a-b
    } else if a > b {
        a-b
    } else {
        0
    }
}

#[test]
fn test_darken() {
    assert_eq!(darken(255, 200), 55);
    assert_eq!(darken(55, 55), 0);
    assert_eq!(darken(255, 255), 0);
    assert_eq!(darken(100, 110), 0);
    assert_eq!(darken(0, 200), 0);
    assert_eq!(darken(0, 50), 0);
}

pub enum Mood {
    Happy,
    Sad,
}