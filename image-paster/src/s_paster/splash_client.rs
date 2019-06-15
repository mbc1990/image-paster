use reqwest::Response;
use std::fs::File;
use rand::Rng;
use image::{DynamicImage, ImageFormat};
use std::io::Read;

pub fn construct_string(strs: &[&str]) -> String {
    let mut ret = String::new();
    for st in strs.iter() {
        ret.push_str(st);
    }
    ret
}

pub struct SplashClient {
    api_key: String
}

impl SplashClient {

    pub fn new(api_key: String) -> SplashClient {
       return SplashClient {
           api_key
       }
    }

    pub fn download_background(&self, query: String) -> Option<DynamicImage> {
        let client = reqwest::Client::new();
        let authorization = construct_string(&["Client-ID ", &self.api_key]);

        let img_query = construct_string(&["https://api.unsplash.com/search/photos?query=\"", query.as_str(), "\""]);

        let mut res = client
            .get(img_query.as_str())
            .header("Authorization", authorization.as_str()) // TODO: Needs api key
            .send().unwrap();

        let v: serde_json::Value = serde_json::from_str(&res.text().unwrap()).unwrap();
        let img_choices = &v["results"].as_array().unwrap();
        let num_choices = img_choices.len();
        if num_choices == 0 {
            return None
        }
        let mut rng = rand::thread_rng();
        let choice = rng.gen_range(0, num_choices-1);
        let matching_image = &v["results"][choice]["links"]["download"];
        println!("Matching img: {:?}", matching_image);


        // Download and return an image
        let mut res2: Response = client
            .get(matching_image.as_str().unwrap())
            .header("Authorization", authorization.as_str())
            .send().unwrap();

        let mut img_bytes = Vec::new();
        res2.copy_to(&mut img_bytes).unwrap();
        let mut background = image::load_from_memory(img_bytes.as_ref()).unwrap();
        return Some(background);
    }
}
