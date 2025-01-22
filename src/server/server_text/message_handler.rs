use super::TextServer;
use common_structs::message::Message::{
    ErrNotFound, ErrUnsupportedRequestType, ReqFile, ReqFilesList, ReqServerType, RespFile,
    RespFilesList, RespServerType,
};
use common_structs::message::{Message, ServerType};

impl TextServer {
    pub(super) fn handle_message(&self, message: Message) -> Message {
        match message {
            ReqServerType => RespServerType(ServerType::Text),
            ReqFilesList => RespFilesList(self.get_files_list()),
            ReqFile(link) => {
                if let Some(file) = self.get_file(&link) {
                    RespFile(file)
                } else {
                    ErrNotFound
                }
            }
            _ => ErrUnsupportedRequestType,
        }
    }
}
