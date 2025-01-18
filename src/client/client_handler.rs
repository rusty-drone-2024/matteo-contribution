use super::TextMediaClientBackend;
use crate::backend::network::PacketMessage;
use crate::backend::ClientNetworkRequest;
use common_structs::message::Message;
use wg_2024::network::SourceRoutingHeader;

impl TextMediaClientBackend {
    pub(crate) fn handle_client_request(&mut self, client_request: ClientNetworkRequest) {
        match client_request {
            ClientNetworkRequest::ListAll(id) => {
                let routing = SourceRoutingHeader::initialize(vec![10, 20]);
                let _ = self
                    .network_send
                    .send(PacketMessage(id, routing, Message::ReqFilesList));
            }
            ClientNetworkRequest::Get(id, _node_id, link) => {
                let routing = SourceRoutingHeader::initialize(vec![10, 20]);
                let _ = self
                    .network_send
                    .send(PacketMessage(id, routing, Message::ReqFile(link)));
            }
        }
    }
}
