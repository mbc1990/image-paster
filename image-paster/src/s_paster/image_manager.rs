use std::fs::File;
use std::io::{BufReader, Read, Write};
use image::{GenericImage, ImageBuffer, RgbImage, GenericImageView, DynamicImage};
use image::{
    ImageFormat,
    imageops::*
};
use s3::credentials::Credentials;
use s3::bucket::Bucket;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use core::iter;
use std::collections::HashMap;

pub struct ImageManager {
    subject_width: u32,
    subject_height: u32,
    subject: image::DynamicImage
}

impl ImageManager {

    pub fn new(fname: String) -> ImageManager {
        println!("Reading subject image");
        let f = File::open(fname).expect("Couldn't load subject image");
        let mut reader = BufReader::new(f);
        let subject = image::load(reader, ImageFormat::PNG).unwrap();
        let (j_width, j_height) = subject.dimensions();
        println!("Subject is {:?} x {:?}", j_width, j_height);
        return ImageManager {
            subject_width: j_width,
            subject_height: j_height,
            subject: subject
        };
    }

    // TODO: This should return an enum that's a sum of all possible image error types, implementing From for each possible error
    pub fn combine(&self, background: &mut DynamicImage) -> String {
        let (width, height) = background.dimensions();
        println!("Background is {:?} x {:?}", width, height);

        // Randomly shrink the subject
        let shrink_factor = rand::thread_rng().gen_range(0.10, 0.65);
        println!("Shrink factor: {:?}", &shrink_factor);
        let resized_w = width as f64 * shrink_factor;
        let resized_h = height as f64 * shrink_factor;

        // Resize subject to fit on background
        let resized = self.subject.resize(resized_w as u32, resized_h as u32, FilterType::Lanczos3);

        // TODO: Do some kind of color/contrast/saturation matching so the subject fits in better
        let kernel = [-1.0f32, -1.0, -1.0,
            -1.0, 8.0, -1.0,
            -1.0, -1.0, -1.0];
        let filtered = background.filter3x3(&kernel);
        filtered.save("/tmp/filtered_bg.png");

        // TODO: x -> y -> count continuous light colored pixels
        // TODO: But this only handles horizontal lines
        /*
        let mut y_runs = HashMap::new();

        for i in 0..filtered.height() {
            let mut x_run = HashMap::new();

            let mut best_run_len = 0;
            let mut cur_run_len = 0;
            let mut best_x = 0;
            let mut best_y = 0;

            for k in 0..filtered.width() {
                let pixel = filtered.get_pixel(i, k);
                println!("Pixel: {:?}", pixel);
                let pixel_data = pixel.data;
                println!("Pixel data: {:?}", pixel_data);
            }
            y_runs.insert(i, x_run);
        }
        */

        println!("Done with edge detection");

        // TODO: Instead of random positioning, do some edge detection and place the subject somewhere that has a horizontal edge to support them
        // Random positioning
        let r_width = resized.width();
        let r_height = resized.height();
        let max_x = width - r_width;
        let max_y = height - r_height;
        let start_x = rand::thread_rng().gen_range(0, max_x);
        let start_y = rand::thread_rng().gen_range(0, max_y);
        println!("Random pos: {:?}, {:?}", start_x, start_y);

        let mut subject_x = 0;
        for i in start_x..start_x + r_width - 1 {
            let mut subject_y = 0;
            for k in start_y..start_y + r_height - 1 {
                let j_pixel = resized.get_pixel(subject_x, subject_y);
                subject_y = subject_y + 1;
                // TODO: Hack around not being able to save alpha channel data
                // TODO: For now, don't copy pixels that are supposed to be transparent
                if j_pixel.data[3] == 0 {
                    continue;
                }
                background.put_pixel(i, k, j_pixel);
            }
            subject_x = subject_x + 1;
        }

        // Resize to reasonable dimensions
        let resized = background.resize(800, 600, FilterType::Lanczos3);

        let credentials = Credentials::default();
        let region = s3::region::Region::UsEast1;
        let bucket = Bucket::new("image-paster", region, credentials).unwrap();

        let mut rng = thread_rng();
        let mut s3_name: String = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(25)
            .collect();

        s3_name.push_str(".png");

        // TODO: Error handling
        let mut encoded_image = Vec::new();
        let (width, height) = resized.dimensions();
        image::png::PNGEncoder::new(encoded_image.by_ref())
            .encode(
                &resized.raw_pixels(),
                width,
                height,
                resized.color()
            ).expect("error encoding pixels as PNG");

        let (res, code) = bucket.put_object(&s3_name, &encoded_image, "multipart/form-data").unwrap();
        println!("Code: {:?}", code);
        println!("res: {:?}", res);

        let mut public_url = "https://image-paster.s3.amazonaws.com/".to_string();
        public_url.push_str(&s3_name);
        return public_url;
    }
}
