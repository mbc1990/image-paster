use slack::{Event, RtmClient, Message};
use rand::distributions::{Distribution, Uniform};
use std::collections::HashMap;
use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::fs;
use std::io::BufReader;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::prelude::SliceRandom;


use super::image_manager::ImageManager;
use super::splash_client::SplashClient;

pub struct Paster {
    ims: Vec<ImageManager>,
    sc: SplashClient
}

impl Paster {
    pub fn new(splash_api_key: String, subject_paths: Vec<String>) -> Paster {
        let mut ims = Vec::new();
        for path in subject_paths {
            ims.push(ImageManager::new(path));
        }
        return Paster {
            ims: ims,
            sc: SplashClient::new(splash_api_key)
        };
    }
}


#[allow(unused_variables)]
impl slack::EventHandler for Paster {
    fn on_event(&mut self, cli: &RtmClient, event: Event) {
        match event {
            Event::Message(msg) => {
                match *msg {
                    Message::Standard(msg) => {
                        println!("msg: {:?}", msg);
                        let text = msg.text.unwrap();

                        println!("text: {:?}", text);
                        if text.contains("<@UJLHVFB6J>") {  // Testing
                        // if text.contains("<@UK1VC3CV8>") {  // Zika
                            println!("Mentioned");
                            let query_start = text.find(" ");
                            match query_start {
                                Some(q_string) => {
                                    let query = &text[q_string+1..text.len()];
                                    println!("Query: {:?}", &query);

                                    // Do the bot thing
                                    let success = self.sc.download_background(query.to_string());
                                    match success {
                                        Some(_) => {
                                            // TODO: Should be randomly selected or from input param
                                            let im = self.ims.first().unwrap();
                                            let public_url = im.combine("/tmp/dl.jpg".to_string());
                                            let channel = msg.channel.unwrap();
                                            let _ = cli.sender().send_message(&channel, &public_url);
                                        },
                                        _ => {
                                            let channel = msg.channel.unwrap();
                                            let _ = cli.sender().send_message(&channel, "Could not find image");
                                        }
                                    }
                                },
                                _ =>{}
                            }
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }
    fn on_close(&mut self, cli: &RtmClient) {
        println!("Connection closed");
    }

    fn on_connect(&mut self, cli: &RtmClient) {
        println!("Paster connected");
    }
}