extern crate image;
use image::GenericImageView;
use std::{env, process::exit};

// Default height and width.
macro_rules! DEF_WH {
    (W) => {
        50
    };
    (H) => {
        5
    }
}

fn parse_args() -> (String, u32, u32) {
    if env::args().len() < 4 {
        println!("Usage: avg_color Filename Width Height");
        exit(1);
    }
    else {
        let x = match env::args().nth(2).unwrap().parse::<u32>() {
            Ok(n) => {
                if n > 0 { n }
                else {
                    println!("Error. Using default width: {}", DEF_WH!(W));
                    DEF_WH!(W) }
            },
            Err(_) => {
                println!("Error. Using default width: {}", DEF_WH!(W));
                DEF_WH!(W)},
        };
        
        let y = match env::args().nth(3).unwrap().parse::<u32>() {
            Ok(n) => {
                if n > 0 { n }
                else {
                    println!("Error. Using default height: {}", DEF_WH!(H));
                    DEF_WH!(H) }
            },
            Err(_) => {
                println!("Error. Using default height: {}", DEF_WH!(H));
                DEF_WH!(H) },
        };
        
        let filename = env::args().nth(1)
            .expect("Error with arguments.").to_string();
        (filename, x, y)
    }
}

fn main() {
    let (mut r, mut g, mut b) = (0u32, 0u32, 0u32);
    let (filename, width, height) = parse_args();
    
    let img = match image::open(filename) {
        Ok(i) => i,
        Err(_) => {
            println!("Invalid image file.");
            exit(1);
        },
    };

    for pixel in img.pixels() {
        let px = &pixel.2;
        r += px[0] as u32;
        g += px[1] as u32;
        b += px[2] as u32;
    }

    let npixels = img.dimensions().0 * img.dimensions().1;
    r /= npixels;
    g /= npixels;
    b /= npixels;
    
    println!("Average: R:{} G:{} B:{}", &r, &g, &b);

    let mut imgbuf = image::RgbImage::new(width, height);
    for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb([r as u8, g as u8, b as u8]);
    }
    imgbuf.save("result.png").unwrap();
}
