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
        msg: Message,
        session: SessionId,
        other_end: NodeId,
    ) -> Option<Message> {
        Some(match msg {
            ReqServerType => RespServerType(ServerType::Text),
            ReqFilesList => RespFilesList(self.get_files_list()),
            ReqFile(link) => RespFile(self.async_get_file(&link, session, other_end)?),
            _ => ErrUnsupportedRequestType,
        })
    }
}
