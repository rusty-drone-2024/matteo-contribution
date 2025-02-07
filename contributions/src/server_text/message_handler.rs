use super::TextServer;
use common_structs::message::Message::{
    ErrNotFound, ErrUnsupportedRequestType, ReqFile, ReqFilesList, ReqServerType, RespFile,
    RespFilesList, RespServerType,
};
use common_structs::message::{FileWithData, Link, ServerType};
use network::PacketMessage;

impl TextServer {
    /// Give a response in the form of a `PacketMessage` based on the given
    /// `packet_msg`. All field except `message` will remain the same.
    /// # Return
    /// The `PacketMessage` which is the response of `packet_msg` given.
    pub(super) fn handle_message(&self, packet_msg: PacketMessage) -> PacketMessage {
        let PacketMessage {
            session,
            opposite_end,
            message,
        } = packet_msg;

        let resp = match message {
            ReqServerType => RespServerType(ServerType::Text(6)),
            ReqFilesList => RespFilesList(self.get_files_list()),
            ReqFile(link) => self.get_file(&link).map_or(ErrNotFound, RespFile),
            _ => ErrUnsupportedRequestType,
        };

        PacketMessage::new(session, opposite_end, resp)
    }

    /// Get the list of links of files the server contains
    /// # Return
    /// A vector containing the Link that the server is willing
    /// to respond to, meaning it contains that file.
    fn get_files_list(&self) -> Vec<Link> {
        self.files.keys().cloned().collect()
    }

    /// Get a file based on the `link` given
    /// # Return
    /// The file if it exist else `None`
    fn get_file(&self, link: &Link) -> Option<FileWithData> {
        self.files.get(link).cloned()
    }
}
