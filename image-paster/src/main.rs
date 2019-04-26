extern crate slack;
extern crate rand;
extern crate serde_json;
extern crate image;
extern crate reqwest;


use crate::paster::Paster;
mod paster;

use std::path::Path;
use slack::RtmClient;
use std::fs::File;
use std::io::BufReader;
use image::{GenericImage, ImageBuffer, RgbImage, GenericImageView};
use image::{
    ImageFormat,
    imageops::*
};
use std::collections::HashMap;
use std::io;


fn main() {
    println!("Reading subject image");
    let f = File::open("/home/malcolm/projects/image-paster/subject.png").expect("Couldn't load subject image");
    let mut reader = BufReader::new(f);
    let subject = image::load(reader, ImageFormat::PNG).unwrap();
    let (j_width, j_height) = subject.dimensions();
    println!("Subject is {:?} x {:?}", j_width, j_height);

    // TODO: Read image from internet
    let background_noun = "cloud";
    
    // TODO: Make request from splash api
    let client = reqwest::Client::new();
    let mut res = client
        .get("https://api.unsplash.com/search/photos?query=\"clouds\"")
        .header("Authorization", "Client-ID ")
        .send().unwrap();

    let v: serde_json::Value = serde_json::from_str(&res.text().unwrap()).unwrap();
    let matching_image = &v["results"][0]["links"]["download"];
    println!("Matching img: {:?}", matching_image);

    let img = reqwest::get(matching_image)
        .unwrap()
        .text()
        .unwrap();

    // TODO: Save image, formatting, etc.


    // println!("{:?}", res.json());

    println!("Reading background image");
    let f2 = File::open("/home/malcolm/projects/image-paster/clouds.png").expect("Couldn't load cloud image");
    let mut reader2 = BufReader::new(f2);
    let mut background = image::load(reader2, ImageFormat::PNG).unwrap();
    let (width, height) = background.dimensions();
    println!("Background is {:?} x {:?}", width, height);

    // TODO: Resize subject randomly within a range

    let mut min_width = j_width;
    if width < j_width {
        min_width = width;
    }

    let mut min_height = j_height;
    if height < j_height {
        min_height = height;
    }

    println!("min dims is {:?} x {:?}", min_width, min_height);

    // Copy minimum matching rectangle of subject into background
    for i in 0..min_width {
        for k in 0..min_height {
            let j_pixel = subject.get_pixel(i, k);

            // TODO: Hack around not being able to save alpha channel data
            // TODO: For now, don't copy pixels that are supposed to be transparent
            if j_pixel.data[3] == 0 {
                continue;
            }
            // println!("{:?}", j_pixel);

            background.put_pixel(i, k, j_pixel);
        }
    }

    background.save("output.png");


    /*
    // TODO: Slack integration
    let args: Vec<String> = std::env::args().collect();
    let api_key = args[1].clone();
    let mut handler = Paster::new(&namespace);
    let r = RtmClient::login_and_run(&api_key, &mut handler);
    match r {
        Ok(_) => {}
        Err(err) => panic!("Error: {}", err),
    }
    */
}