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
    pub fn new(entry: &fs::DirEntry) -> ImageFile {
        let filename = entry.file_name().to_str().unwrap().to_owned();
        let image = ImageType::Dynamic(image::open(&entry.path()).unwrap());

        ImageFile { image, filename }
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

pub fn load_images(path: &Path) -> Result<Vec<ImageFile>, FileError> {
    match fs::read_dir(path) {
        Ok(files) => {
            Ok(files.map(|file| file.unwrap())
                    .filter(|entry| allowed_filetype(entry))
                    .map(|entry| ImageFile::new(&entry))
                    .collect::<Vec<ImageFile>>())
        },

        Err(err) => Err(FileError::NoFile(err))
    }
}