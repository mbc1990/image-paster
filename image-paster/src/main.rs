mod s_paster;

use std::fs::*;
use std::io::prelude::*;

extern crate slack;
extern crate rand;
extern crate serde_json;
extern crate image;
extern crate reqwest;
extern crate s3;

use crate::image::Pixel;

use s_paster::paster::Paster;

use std::fs;
use std::path::Path;
use slack::RtmClient;
use std::io::BufReader;
use image::{GenericImage, ImageBuffer, RgbImage, GenericImageView};
use image::{
    ImageFormat,
    imageops::*
};
use std::collections::HashMap;
use std::io;
use reqwest::Response;
use rand::Rng;


fn main() {
    let args: Vec<String> = std::env::args().collect();
    let slack_api_key = args[1].clone();
    let splash_api_key = args[2].clone();
    let subject_path = args[3].clone();
    let bot_id = args[4].clone();
    let paths = fs::read_dir(&subject_path).unwrap();
    let mut subject_paths = Vec::new();
    for path in paths {
        let extracted_path = path.unwrap().path();
        let path_str = extracted_path.to_str().unwrap();
        subject_paths.push(path_str.to_string());
    }
    /*
    println!("Doing convolution pass");
    let kernel = [-1.0f32, -1.0, -1.0,
        -1.0, 8.0, -1.0,
        -1.0, -1.0, -1.0];
        */
    /*
    let mut img = image::open("/home/malcolm/projects/image-paster/subjects/drew.png".to_string()).unwrap();
    let fire = image::open("/home/malcolm/projects/image-paster/Fire.jpg".to_string()).unwrap();
    // img.invert();
    let (w, h) = img.dimensions();
    println!("Image is {:?}x{:?}", w, h);
    let (f_w, f_h) = fire.dimensions();
    println!("fire is {:?}x{:?}", f_w, f_h);

    // Rearrange image
    let sq_size = w / 4;

    let num_swaps = rand::thread_rng().gen_range(10, 20);
    for i in 0..num_swaps {
        let from_x = rand::thread_rng().gen_range(0, w - sq_size - 1);
        let from_y = rand::thread_rng().gen_range(0, h - sq_size - 1);

        let to_x = rand::thread_rng().gen_range(0, w - sq_size - 1);
        let to_y = rand::thread_rng().gen_range(0, h - sq_size - 1);
        let should_invert = rand::thread_rng().gen_range(0, 10);
        for x in 0..sq_size {
            for y in 0..sq_size {
                let mut tmp_px = img.get_pixel(from_x + x, from_y + y);
                let mut swap = img.get_pixel(to_x + x, to_y + y);

                if should_invert < 2 {
                    tmp_px.invert();
                    swap.invert();
                }

                img.put_pixel(from_x + x, from_y + y, swap);
                img.put_pixel(to_x + x, to_y + y, tmp_px);
            }
        }
    }

    // Blend with fire
    for x in 0..w {
        for y in 0..h {
            let fire_px = fire.get_pixel(x, y);
            let mut img_px = img.get_pixel(x, y);

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
            img.put_pixel(x, y, blended_px);
        }
    }

    img.save("/tmp/filtered_bg.png");

    return;
    */

    let mut handler = Paster::new(splash_api_key, subject_paths, bot_id);
    let r = RtmClient::login_and_run(&slack_api_key, &mut handler);
    match r {
        Ok(_) => {}
        Err(err) => panic!("Error: {}", err),
    }

}