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
                Self::get_test_media(&link).map_or(ErrNotFound, RespMedia)
            }
            _ => ErrUnsupportedRequestType,
        }
    }
}
