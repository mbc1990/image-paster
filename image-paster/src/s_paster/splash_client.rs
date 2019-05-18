use reqwest::Response;
use std::fs::File;

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
    pub fn download_background(query: String) -> String {
        return String::new();
        let client = reqwest::Client::new();
        let mut res = client
            .get("https://api.unsplash.com/search/photos?query=\"nebraska\"")
            .header("Authorization", "Client-ID ") // TODO: Needs api key
            .send().unwrap();

        let v: serde_json::Value = serde_json::from_str(&res.text().unwrap()).unwrap();
        let matching_image = &v["results"][0]["links"]["download"];
        println!("Matching img: {:?}", matching_image);

        let mut res2: Response = client
            .get(matching_image.as_str().unwrap())
            .header("Authorization", "Client-ID ") // TODO: Needs api key
            .send().unwrap();

        // Works! - Write downloaded image to file
        // TODO: Should generate a unique name so we can store backgrounds
        let mut f = File::create("/tmp/dl.jpg").expect("Unable to create file");
        res2.copy_to(&mut f).unwrap();
        // end works
        println!("RES2: {:?}", res2);
        println!("Reading background image");
        return "/tmp/dl.jpg".to_string();
    }
}
