use super::TextServer;
use common_structs::message::Message::*;
use common_structs::message::{Message, ServerType};

impl TextServer {
    pub(super) fn handle_message(&self, message: Message) -> Message {
        match message.clone() {
            ReqServerType => RespServerType(ServerType::Text),
            ReqFilesList => RespFilesList(self.get_files_list()),
            ReqFile(link) => {
                if let Some(file) = self.get_file(link) {
                    RespFile(file)
                } else {
                    ErrNotFound
                }
            }
            _ => ErrUnsupportedRequestType,
        }
    }
}
