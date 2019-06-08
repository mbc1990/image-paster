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
    let mut handler = Paster::new(splash_api_key, subject_path);
    let r = RtmClient::login_and_run(&slack_api_key, &mut handler);
    match r {
        Ok(_) => {}
        Err(err) => panic!("Error: {}", err),
    }

}