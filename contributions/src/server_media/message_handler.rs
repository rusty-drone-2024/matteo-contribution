use super::MediaServer;
use common_structs::message::Message::{
    ErrNotFound, ErrUnsupportedRequestType, ReqMedia, ReqServerType, RespMedia, RespServerType,
};
use common_structs::message::{Message, ServerType};

impl MediaServer {
    pub(crate) fn handle_test_message(message: Message) -> Message {
        match message {
            ReqServerType => RespServerType(ServerType::Media(7)),
            ReqMedia(link) => {
                if let Some(media) = Self::get_test_media(&link) {
                    RespMedia(media)
                } else {
                    ErrNotFound
                }
            }
            _ => ErrUnsupportedRequestType,
        }
    }
}
