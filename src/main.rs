extern crate image;
mod lib;

use std::path::Path;
use std::time::Instant;

use lib::*;

const ASPECT_RATIO: f32 = 0.75;

fn main() {
    let start = Instant::now();
    let (path, output_path) = (Path::new("images"), Path::new("resized-images"));
    match load_images(path) {
        Ok(images) => for image in images { resize_image(image, output_path); },
        _ => println!("Failed to load images."),
    }

    let elapsed = start.elapsed();
    println!("Resizing took {} ms.",
             (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);
}

fn resize_image(image: ImageFile, output_path: &Path) {
    let result_dimensions = scale_to_aspect_ratio(ASPECT_RATIO, &image.dimensions());
    let offset = get_center_offset(&result_dimensions, &image.dimensions());
    let resized_image = image.create_resized(&result_dimensions, &offset);

    match resized_image {
        Ok(image) => image.save(output_path),
        Err(err) => { println!("{:?}", err); }
    }
}

fn get_center_offset(first: &Dimension, second: &Dimension) -> Point {
    Point {
        x: (first.width - second.width) / 2,
        y: (first.height - second.height) / 2
    }
}

fn scale_to_aspect_ratio(aspect_ratio: f32, dimensions: &Dimension) -> Dimension {
    let quotient = dimensions.width as f32 / dimensions.height as f32;

    match quotient < aspect_ratio {
        true => Dimension {
            width: (dimensions.height as f32 * ASPECT_RATIO).floor() as u32,
            height: dimensions.height
        },
        false => Dimension {
            width: dimensions.width, 
            height: (dimensions.width as f32 * (1.0 / ASPECT_RATIO)).floor() as u32
        }
    }
}