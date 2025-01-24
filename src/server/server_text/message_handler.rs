use super::TextServer;
use common_structs::message::Message::{
    ErrUnsupportedRequestType, ReqFile, ReqFilesList, ReqServerType, RespFile, RespFilesList,
    RespServerType,
};
use common_structs::message::{Message, ServerType};
use common_structs::types::SessionId;
use wg_2024::network::NodeId;

impl TextServer {
    pub(super) fn handle_message(
        &self,
        message: Message,
        session_id: SessionId,
        other_end: NodeId,
    ) -> Option<Message> {
        Some(match message {
            ReqServerType => RespServerType(ServerType::Text),
            ReqFilesList => RespFilesList(self.get_files_list()),
            ReqFile(link) => RespFile(self.async_get_file(&link, session_id, other_end)?),
            _ => ErrUnsupportedRequestType,
        })
    }
}
