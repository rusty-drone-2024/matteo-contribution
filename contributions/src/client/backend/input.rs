use crate::client::backend::ClientBackend;
use client_bridge::{GuiRequest, RequestWrapper};
use common_structs::message::Message::{ReqFile, ReqFilesList, ReqMedia};
use common_structs::message::{Message, ServerType};
use common_structs::types::Session;
use network::PacketMessage;
use wg_2024::network::NodeId;
use wg_2024::packet::NodeType;

impl ClientBackend {
    pub(super) fn handle_frontend_request(&mut self, mut rq: RequestWrapper) {
        let res = self.handle_frontend_rq_types(&mut rq);

        let Some(session) = res else {
            rq.post_err_not_found();
            return;
        };

        self.open_requests.insert(session, rq);
    }

    fn handle_frontend_rq_types(&mut self, rq: &mut RequestWrapper) -> Option<Session> {
        let session = self.fresh_session();

        match rq.take_request()? {
            GuiRequest::ListAll => {
                let mut count = 0;

                // TODO remove clone
                for (id, server_type) in self.servers.clone() {
                    if let Some(ServerType::Text) = server_type {
                        let part_session_id = self.fresh_session();
                        let packet_msg = PacketMessage::new(part_session_id, id, ReqFilesList);
                        self.split_req.insert(part_session_id, session);
                        let _ = self.network_send.send(packet_msg);

                        count += 1;
                    }
                }

                // TODO Stop assuming that is already done topology
                if count == 0 {
                    return None;
                }

                self.accumulator_list_all.insert(session, (count, vec![]));
            }
            GuiRequest::Get(link) => {
                let server_id = self.get_from_dns(&link)?;
                let packet_msg = PacketMessage::new(session, server_id, ReqFile(link));
                let _ = self.network_send.send(packet_msg);
            }
            GuiRequest::GetMedia(link) => {
                println!("DNS {:?}", self.dns);
                let server_id = self.get_from_dns(&link)?;
                let packet_msg = PacketMessage::new(session, server_id, ReqMedia(link));
                let _ = self.network_send.send(packet_msg);
            }
        }

        Some(session)
    }

    pub(super) fn handle_new_leaf(&mut self, node_id: NodeId, node_type: NodeType) {
        if node_type == NodeType::Server {
            self.servers.push((node_id, None));

            let packet_req_type =
                PacketMessage::new(self.fresh_session(), node_id, Message::ReqServerType);

            let _ = self.network_send.send(packet_req_type);
        }
    }

    fn fresh_session(&mut self) -> u64 {
        let res = self.new_session;
        self.new_session += 1;
        res
    }
}
