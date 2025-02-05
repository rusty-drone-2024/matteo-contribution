use std::fs::read;
use std::path::Path;
use super::MediaServer;
use common_structs::message::{Link, Media};

impl MediaServer {
    pub(crate) fn get_test_media(link: &Link) -> Option<Media> {
        println!("CHICKEN: {link}");
        if link == "chicken.jpeg" {
            if let Ok(image) = read(Path::new("chicken.jpeg")){
                return Some(image);
            } 
            eprintln!("Coudn't load mistic image of chicken");
        }
        None
    }
}
