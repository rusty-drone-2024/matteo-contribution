use super::MediaServer;
use common_structs::message::{Link, Media};
use std::fs::read;
use std::path::Path;

const IMAGE_PATH: &str = ".resources/img_matteo";

impl MediaServer {
    pub(crate) fn get_test_media(link: &Link) -> Option<Media> {
        match link.as_str() {
            "chicken.jpeg" | "sunset.jpg" | "ferris.png" => {}
            _ => {
                return None;
            }
        }

        let link = &format!("{IMAGE_PATH}/{link}");
        let Ok(media) = read(Path::new(link)) else {
            eprintln!("Images of Matteo Media Server are missing");
            return None;
        };

        Some(media)
    }
}
