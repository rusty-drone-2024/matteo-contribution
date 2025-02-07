use super::MediaServer;
use common_structs::message::Message::{
    ErrNotFound, ErrUnsupportedRequestType, ReqMedia, ReqServerType, RespMedia, RespServerType,
};
use common_structs::message::{Link, Media, Message, ServerType};
use std::fs::read;
use std::path::Path;

impl MediaServer {
    pub(super) fn handle_test_message(message: Message) -> Message {
        match message {
            ReqServerType => RespServerType(ServerType::Media(7)),
            ReqMedia(link) => Self::get_test_media(&link).map_or(ErrNotFound, RespMedia),
            _ => ErrUnsupportedRequestType,
        }
    }

    fn get_test_media(link: &Link) -> Option<Media> {
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
