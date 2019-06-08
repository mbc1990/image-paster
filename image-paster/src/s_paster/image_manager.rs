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

        let width_start = width / 3 - self.subject_width/2;
        let height_start = height / 2 - self.subject_height/2;

        // TODO: Resize subject randomly within a range
        let mut min_width = self.subject_width;
        if width < self.subject_width {
            min_width = width;
        }

        let mut min_height = self.subject_height;
        if height < self.subject_height {
            min_height = height;
        }

        println!("min dims is {:?} x {:?}", min_width, min_height);
        println!("Width start {:?}", width_start);

        // Copy minimum matching rectangle of subject into background
        for i in 0..min_width {
            for k in 0..min_height {
                let j_pixel = self.subject.get_pixel(i, k);

                // TODO: Hack around not being able to save alpha channel data
                // TODO: For now, don't copy pixels that are supposed to be transparent
                if j_pixel.data[3] == 0 {
                    continue;
                }
                // println!("{:?}", j_pixel);

                background.put_pixel(i + width_start, k + height_start, j_pixel);
            }
        }

        // Resize to reasonable dimensions
        let resized = background.resize(800, 600, FilterType::Nearest);

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
