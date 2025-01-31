use super::TextServer;
use common_structs::message::Message::{
    ErrUnsupportedRequestType, ReqFile, ReqFilesList, ReqServerType, RespFile, RespFilesList,
    RespServerType,
};
use common_structs::message::{FileWithData, Link, Message, ServerType};

impl TextServer {
    pub(super) fn handle_message(&self, msg: Message) -> Option<Message> {
        Some(match msg {
            ReqServerType => RespServerType(ServerType::Text),
            ReqFilesList => RespFilesList(self.get_files_list()),
            ReqFile(link) => RespFile(self.get_file(&link)?),
            _ => ErrUnsupportedRequestType,
        })
    }

    fn get_files_list(&self) -> Vec<Link> {
        self.files.keys().cloned().collect()
    }

    fn get_file(&self, link: &Link) -> Option<FileWithData> {
        self.files.get(link).cloned()
    }
}
