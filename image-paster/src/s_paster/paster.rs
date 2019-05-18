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

pub struct Paster {
    im: ImageManager
}

impl Paster {
    pub fn new(splash_api_key: String) -> Paster {
        return Paster {
            im: ImageManager::new("/home/malcolm/projects/image-paster/subject.png".to_string())
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
                        if text.contains("<@UJLHVFB6J>") {
                            println!("Mentioned");
                            // TODO: This will panic if you only @<botid> with no query
                            let query_start = text.find(" ").expect("Couldn't parse bot query");
                            let query = &text[query_start+1..text.len()];

                            // TODO: Call the image manipulation logic
                            println!("Query: {:?}", query);
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