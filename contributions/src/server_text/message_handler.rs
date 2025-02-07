use super::TextServer;
use common_structs::message::Message::{
    ErrNotFound, ErrUnsupportedRequestType, ReqFile, ReqFilesList, ReqServerType, RespFile,
    RespFilesList, RespServerType,
};
use common_structs::message::{FileWithData, Link, ServerType};
use network::PacketMessage;

impl TextServer {
    pub(super) fn handle_message(&self, packet_msg: PacketMessage) -> Option<PacketMessage> {
        let PacketMessage {
            session,
            opposite_end,
            message,
        } = packet_msg;

        let response = match message {
            ReqServerType => RespServerType(ServerType::Text(6)),
            ReqFilesList => RespFilesList(self.get_files_list()),
            ReqFile(link) => self.get_file(&link).map_or(ErrNotFound, RespFile),
            _ => ErrUnsupportedRequestType,
        };

        Some(PacketMessage::new(session, opposite_end, response))
    }

    fn get_files_list(&self) -> Vec<Link> {
        self.files.keys().cloned().collect()
    }

    fn get_file(&self, link: &Link) -> Option<FileWithData> {
        self.files.get(link).cloned()
    }
}
