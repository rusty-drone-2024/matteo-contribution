use super::MediaServer;
use common_structs::message::Message::{
    ErrNotFound, ErrUnsupportedRequestType, ReqMedia, ReqServerType, RespMedia, RespServerType,
};
use common_structs::message::{Link, Media, ServerType};
use network::PacketMessage;
use std::fs::read;
use std::path::Path;

impl MediaServer {
    /// Give a response in the form of a `PacketMessage` based on the given
    /// `packet_msg`. All field except `message` will remain the same.
    /// # Return
    /// The `PacketMessage` which is the response of `packet_msg` given.
    pub(super) fn handle_message(packet_msg: PacketMessage) -> PacketMessage {
        let PacketMessage {
            session,
            opposite_end,
            message,
        } = packet_msg;

        let resp = match message {
            ReqServerType => RespServerType(ServerType::Media(7)),
            ReqMedia(link) => Self::get_media(&link).map_or(ErrNotFound, RespMedia),
            _ => ErrUnsupportedRequestType,
        };

        PacketMessage::new(session, opposite_end, resp)
    }

    /// Get `Media` based on the `link` given
    /// # Return
    /// The given `Media` if it exist. Else `None` is returned.
    fn get_media(link: &Link) -> Option<Media> {
        match link.as_str() {
            "my-chicken.jpeg" | "sunset.jpg" | "ferris.png" => {}
            _ => return None,
        }

        let link = &format!(".resources/img_matteo/{link}");
        let Ok(media) = read(Path::new(link)) else {
            eprintln!("Images of Matteo Media Server are missing");
            return None;
        };

        Some(media)
    }
}
