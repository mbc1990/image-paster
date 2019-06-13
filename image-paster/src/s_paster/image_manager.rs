use std::fs::File;
use std::io::{BufReader, Read};
use image::{GenericImage, ImageBuffer, RgbImage, GenericImageView};
use image::{
    ImageFormat,
    imageops::*
};
use s3::credentials::Credentials;
use s3::bucket::Bucket;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use core::iter;

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
    pub fn combine(&self, background_img_path: String) -> String {
        let f2 = File::open("/tmp/dl.jpg").expect("Couldn't load tmp image");
        let mut reader2 = BufReader::new(f2);
        let mut background = image::load(reader2, ImageFormat::JPEG).unwrap();
        let (width, height) = background.dimensions();
        println!("Background is {:?} x {:?}", width, height);

        // Randomly shrink the subject
        let shrink_factor = rand::thread_rng().gen_range(0.10, 0.65);
        println!("Shrink factor: {:?}", &shrink_factor);
        let resized_w = width as f64 * shrink_factor;
        let resized_h = height as f64 * shrink_factor;

        // Resize subject to fit on background
        let resized = self.subject.resize(resized_w as u32, resized_h as u32, FilterType::Lanczos3
        );
        let r_width = resized.width();
        let r_height = resized.height();

        // Random positioning
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

        resized.save("/tmp/output.png");

        let credentials = Credentials::default();
        let region = s3::region::Region::UsEast1;
        let bucket = Bucket::new("image-paster", region, credentials).unwrap();
        let mut to_upload = File::open("/tmp/output.png").unwrap();

        let mut data = Vec::new();
        to_upload.read_to_end(&mut data).expect("Unable to read data");

        let mut rng = thread_rng();
        let mut s3_name: String = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(25)
            .collect();

        s3_name.push_str(".png");

        let (res, code) = bucket.put_object(&s3_name, &data, "multipart/form-data").unwrap();
        println!("Code: {:?}", code);
        println!("res: {:?}", res);

        let mut public_url = "https://image-paster.s3.amazonaws.com/".to_string();
        public_url.push_str(&s3_name);
        return public_url;
    }
}
