use super::MediaServer;
use common_structs::message::{Link, Media};
use std::fs::read;
use std::path::Path;

const IMAGE_PATH: &str = ".resources/img_matteo";

impl MediaServer {
    pub(crate) fn get_test_media(link: &Link) -> Option<Media> {
        let link = &format!("{IMAGE_PATH}/{link}");
        
        if let Ok(image) = read(Path::new(link)) {
            return Some(image);
        }

        eprintln!("Coudn't load mistic image of {link}");
        None
    }
}
