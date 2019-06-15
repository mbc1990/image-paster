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
use rand::Rng;


use super::image_manager::ImageManager;
use super::splash_client::SplashClient;
use std::error::Error;

pub struct Paster {
    ims: Vec<ImageManager>,
    sc: SplashClient,
    bot_id: String
}

pub enum PasterError {
    CouldNotFindImage
}

impl Paster {
    pub fn new(splash_api_key: String, subject_paths: Vec<String>, bot_id: String) -> Paster {
        let mut ims = Vec::new();
        for path in subject_paths {
            ims.push(ImageManager::new(path));
        }
        return Paster {
            ims: ims,
            sc: SplashClient::new(splash_api_key),
            bot_id: bot_id
        };
    }

    fn handle_mention(&self, text: String, channel: String, cli: &RtmClient) -> Result<(), PasterError> {
        let query_start = text.find(" ");
        match query_start {
            Some(q_string) => {
                let query = &text[q_string+1..text.len()];
                println!("Query: {:?}", &query);

                // Do the bot thing
                let success = self.sc.download_background(query.to_string());
                match success {
                    Some(_) => {
                        let mut rng = rand::thread_rng();
                        println!("{:?} images loaded", self.ims.len());
                        let subject_idx = rng.gen_range(0, self.ims.len());
                        println!("Subject index: {:?}", subject_idx);

                        let im = self.ims.get(subject_idx as usize).unwrap();
                        let public_url = im.combine("/tmp/dl.jpg".to_string());
                        let _ = cli.sender().send_message(&channel, &public_url);
                    },
                    _ => {
                        let _ = cli.sender().send_message(&channel, "Could not find image");
                    }
                }
            },
            _ =>{}
        }
        Ok(())
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
                        println!("Bot id: {:?}", &self.bot_id);
                        if text.contains(&self.bot_id) {
                            println!("Mentioned");
                            let channel = msg.channel.unwrap();
                            self.handle_mention(text, channel, cli);
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