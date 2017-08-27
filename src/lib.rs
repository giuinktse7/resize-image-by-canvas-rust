extern crate image;

use std::fs;
use std::io;
use std::fs::File;
use std::path::Path;
use image::{ DynamicImage, GenericImage, ImageBuffer };


enum ImageType {
    Dynamic(DynamicImage),
    Buffer(ImageBuffer<image::Rgba<u8>, Vec<u8>>)
}

pub struct ImageFile {
    image: ImageType,
    filename: String
}

impl ImageFile {
    pub fn new(entry: &fs::DirEntry) -> Result<ImageFile, image::ImageError> {
        let filename = entry.file_name().to_str().unwrap().to_owned();
        let image_file = image::open(&entry.path());

        match image_file {
            Ok(image) => Ok(ImageFile { image: ImageType::Dynamic(image), filename }),
            Err(err) => Err(err)
        }
    }

    pub fn dimensions(&self) -> Dimension {
        match self.image {
            ImageType::Dynamic(ref image) => {
                let (width, height) = image.dimensions();
                Dimension { width, height }
            },
            ImageType::Buffer(ref image) => {
                let (width, height) = image.dimensions();
                Dimension { width, height }
            }
        }
    }

    pub fn create_resized(&self, dimensions: &Dimension, offset: &Point) -> Result<ImageFile, FileError> {
        let mut result = ImageBuffer::new(dimensions.width, dimensions.height);
        match self.image {
            ImageType::Dynamic(ref image) => {
                result.copy_from(image, offset.x, offset.y);

                Ok(ImageFile {
                    image: ImageType::Buffer(result),
                    filename: self.filename.clone()
                })
            },

            _ => Err(FileError::Panic)
        }
    }

    pub fn save(&self, output_path: &Path) {
        let save_path = &Path::new(output_path).join(&self.filename);
        create_file(output_path, &self.filename);
        match self.image {
            ImageType::Buffer(ref image) => {
                image.save(save_path).unwrap();
            },
            _ => println!("Can only save Images of type ImageBuffer.")
        }
    }


}

pub struct Point {
    pub x: u32,
    pub y: u32
}

#[derive(Debug)]
pub enum FileError {
    NoFile(io::Error),
    Panic
}

impl From<io::Error> for FileError {
    fn from(error: io::Error) -> Self {
        FileError::NoFile(error)
    }
}

pub struct Dimension {
    pub width: u32,
    pub height: u32
}

impl Dimension {
    pub fn to_aspect_ratio(&self, aspect_ratio: f32) -> Self {
        let quotient = self.width as f32 / self.height as f32;
        if quotient < aspect_ratio {
            Dimension {
                width: (self.height as f32 * aspect_ratio).floor() as u32,
                height: self.height
            }
        } else {
            Dimension {
                width: self.width, 
                height: (self.width as f32 * (1.0 / aspect_ratio)).floor() as u32
            }
        }
    }
}

fn create_file(path: &Path, filename: &str) {
    if !path.exists() {
        fs::create_dir(path).unwrap();        
    }

    let file_path = path.join(filename).as_path().to_owned();
    if !file_path.exists() {
        File::create(file_path).unwrap();
    }
}


fn allowed_filetype(entry: &fs::DirEntry) -> bool {
    match entry.path().extension() {
        Some(extension) => {
            match extension.to_str().unwrap() {
                "jpg" | "jpeg" | "png" => true,
                _ => false
            }
        },
        None => false
    }
}

pub fn load_images(path: &Path) -> Vec<ImageFile> {
    match fs::read_dir(path) {
        Ok(files) => {
            files.map(|file| file.unwrap())
                    .filter(|entry| allowed_filetype(entry))
                    .map(|entry| ImageFile::new(&entry))
                    .filter_map(|result| result.ok())
                    .collect()
        },

        Err(_) => {
            fs::create_dir(path).unwrap();
            vec![]
        }
    }
}