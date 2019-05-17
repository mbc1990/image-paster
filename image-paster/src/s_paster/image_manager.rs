use std::fs::File;
use std::io::BufReader;
use image::{GenericImage, ImageBuffer, RgbImage, GenericImageView};
use image::{
    ImageFormat,
    imageops::*
};

pub struct ImageManager {
    subject_width: u32,
    subject_height: u32,
    subject: image::DynamicImage
}

impl ImageManager {
    pub fn new(fname: String) -> ImageManager {
        println!("Reading subject image");
        let f = File::open("/home/malcolm/projects/image-paster/subject.png").expect("Couldn't load subject image");
        let mut reader = BufReader::new(f);
        let subject = image::load(reader, ImageFormat::PNG).unwrap();
        let (j_width, j_height) = subject.dimensions();
        println!("Subject is {:?} x {:?}", j_width, j_height);
        return ImageManager{
            subject_width: j_width,
            subject_height: j_height,
            subject: subject
        };
    }
}
