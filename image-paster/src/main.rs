mod s_paster;

use std::fs::*;
use std::io::prelude::*;

extern crate slack;
extern crate rand;
extern crate serde_json;
extern crate image;
extern crate reqwest;
extern crate s3;


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


fn main() {
    let args: Vec<String> = std::env::args().collect();
    let slack_api_key = args[1].clone();
    let splash_api_key = args[2].clone();
    let subject_path= args[3].clone();
    let bot_id = args[4].clone();
    let paths = fs::read_dir(&subject_path).unwrap();
    let mut subject_paths = Vec::new();
    for path in paths {
        let extracted_path = path.unwrap().path();
        let path_str= extracted_path.to_str().unwrap();
        subject_paths.push(path_str.to_string());
    }

    // TODO: Do some kind of color/contrast/saturation matching so the subject fits in better
    println!("Doing convolution pass");
    let kernel = [-1.0f32, -1.0, -1.0,
        -1.0, 8.0, -1.0,
        -1.0, -1.0, -1.0];
    let img = image::open("/home/malcolm/Downloads/nyc.jpg".to_string()).unwrap();
    let filtered = img.filter3x3(&kernel);
    filtered.save("/tmp/filtered_bg.png");
    return;

    let mut handler = Paster::new(splash_api_key, subject_paths, bot_id);
    let r = RtmClient::login_and_run(&slack_api_key, &mut handler);
    match r {
        Ok(_) => {}
        Err(err) => panic!("Error: {}", err),
    }

}