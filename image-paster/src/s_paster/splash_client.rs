use reqwest::Response;
use std::fs::File;
use rand::Rng;

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

    // TODO: Should return a result type
    pub fn download_background(&self, query: String) -> String {
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
        let mut rng = rand::thread_rng();
        let choice = rng.gen_range(0, num_choices-1);
        let matching_image = &v["results"][choice]["links"]["download"];
        println!("Matching img: {:?}", matching_image);

        let mut res2: Response = client
            .get(matching_image.as_str().unwrap())
            .header("Authorization", authorization.as_str()) // TODO: Needs api key
            .send().unwrap();

        // Write downloaded image to file
        // TODO: Should generate a unique name so we can store backgrounds
        let mut f = File::create("/tmp/dl.jpg").expect("Unable to create file");
        res2.copy_to(&mut f).unwrap();
        // end works
        println!("RES2: {:?}", res2);
        println!("Reading background image");
        return "/tmp/dl.jpg".to_string();
    }
}
