use crate::image::Pixel;
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

const HELLIFY_PROBABILITY: f32 = 0.25;

pub struct ImageManager {
    subject_width: u32,
    subject_height: u32,
    subject: image::DynamicImage,
    hell_background: image::DynamicImage
}

impl ImageManager {

    pub fn new(fname: String, hell_bg_name: String) -> ImageManager {
        println!("Reading subject image");
        let f = File::open(fname).expect("Couldn't load subject image");
        let mut reader = BufReader::new(f);
        let subject = image::load(reader, ImageFormat::PNG).unwrap();
        let (j_width, j_height) = subject.dimensions();
        println!("Subject is {:?} x {:?}", j_width, j_height);
        println!("hell bg path {:?} ",  hell_bg_name);
        let hell_background= image::open(hell_bg_name).unwrap();
        return ImageManager {
            subject_width: j_width,
            subject_height: j_height,
            subject: subject,
            hell_background
        };
    }

    // TODO: This should return an enum that's a sum of all possible image error types, implementing From for each possible error
    pub fn combine(&self, background: &mut DynamicImage) -> String {
        let (width, height) = background.dimensions();
        println!("Background is {:?} x {:?}", width, height);

        // Randomly shrink the subject
        let shrink_factor = rand::thread_rng().gen_range(0.75, 0.85);
        println!("Shrink factor: {:?}", &shrink_factor);
        let resized_w = width as f64 * shrink_factor;
        let resized_h = height as f64 * shrink_factor;

        // Resize subject to fit on background
        let mut resized = self.subject.resize(resized_w as u32, resized_h as u32, FilterType::Lanczos3);
        let (subject_resized_w, subject_resized_h) = resized.dimensions();

        // Small chance of applying hellify effect to the subject
        let should_hellify = rand::thread_rng().gen_range(0.0, 1.0);
        if should_hellify < HELLIFY_PROBABILITY {
            let (f_w, f_h) = self.hell_background.dimensions();
            println!("fire is {:?}x{:?}", f_w, f_h);

            // Rearrange image
            let sq_size = subject_resized_w as u32 / 4;

            let num_swaps = rand::thread_rng().gen_range(10, 20);
            for i in 0..num_swaps {
                let from_x = rand::thread_rng().gen_range(0, subject_resized_w as u32 - sq_size - 1);
                let from_y = rand::thread_rng().gen_range(0, subject_resized_h as u32 - sq_size - 1);

                let to_x = rand::thread_rng().gen_range(0, subject_resized_w as u32 - sq_size - 1);
                let to_y = rand::thread_rng().gen_range(0, subject_resized_h as u32 - sq_size - 1);
                let should_invert = rand::thread_rng().gen_range(0, 10);
                for x in 0..sq_size {
                    for y in 0..sq_size {
                        let mut tmp_px = resized.get_pixel(from_x + x, from_y + y);
                        let mut swap = resized.get_pixel(to_x + x, to_y + y);

                        if should_invert < 2 {
                            tmp_px.invert();
                            swap.invert();
                        }

                        resized.put_pixel(from_x + x, from_y + y, swap);
                        resized.put_pixel(to_x + x, to_y + y, tmp_px);
                    }
                }
            }

            // Blend with fire
            for x in 0..subject_resized_w as u32 {
                for y in 0..subject_resized_h as u32 {
                    let hell_x = x % self.hell_background.width();
                    let hell_y = y % self.hell_background.height();
                    let fire_px = self.hell_background.get_pixel(hell_x, hell_y);
                    let mut img_px = resized.get_pixel(x, y);

                    let i_r= img_px.data[0] as u32;
                    let i_g= img_px.data[1] as u32;
                    let i_b= img_px.data[2] as u32;

                    let f_r = fire_px.data[0] as u32;
                    let f_g = fire_px.data[1] as u32;
                    let f_b = fire_px.data[2] as u32;

                    let b_r = ((i_r + f_r) / 2) as u8;
                    let b_g = ((i_g + f_g) / 2) as u8;
                    let b_b = ((i_b + f_b) / 2) as u8;
                    let mut blended_px = image::Rgba([b_r, b_g, b_b, img_px.data[3]]);
                    println!("blended {:?}", blended_px);
                    resized.put_pixel(x, y, blended_px);
                }
            }

        }

        // Randomly rotate the subject


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
