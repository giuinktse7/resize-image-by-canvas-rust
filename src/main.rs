extern crate image;
mod lib;

use std::path::Path;
use std::time::Instant;

use lib::*;

const ASPECT_RATIO: f32 = 0.75;

fn main() {
    let start = Instant::now();
    let (path, output_path) = (Path::new("images"), Path::new("resized-images"));

    load_images(path).iter()
                     .map(|image| resize_image(image))
                     .filter_map(|result| result.ok())
                     .fold((), |_, image| image.save(output_path));

    let elapsed = start.elapsed();
    println!("Resizing took {} ms.",
             (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);
}

fn resize_image(image: &ImageFile) -> Result<ImageFile, FileError> {
    let result_dimensions = image.dimensions().to_aspect_ratio(ASPECT_RATIO);
    let offset = get_center_offset(&result_dimensions, &image.dimensions());

    image.create_resized(&result_dimensions, &offset)
}

fn get_center_offset(first: &Dimension, second: &Dimension) -> Point {
    Point {
        x: (first.width - second.width) / 2,
        y: (first.height - second.height) / 2
    }
}