use super::MediaServer;
use common_structs::message::Message::{
    ErrNotFound, ErrUnsupportedRequestType, ReqFile, ReqServerType, RespMedia, RespServerType,
};
use common_structs::message::{Message, ServerType};

impl MediaServer {
    pub(super) fn handle_test_message(message: Message) -> Message {
        match message {
            ReqServerType => RespServerType(ServerType::Media),
            // TODO req media
            ReqFile(_link) => {
                if let Some(media) = Self::get_test_media() {
                    RespMedia(media)
                } else {
                    ErrNotFound
                }
            }
            _ => ErrUnsupportedRequestType,
        }
    }
}
