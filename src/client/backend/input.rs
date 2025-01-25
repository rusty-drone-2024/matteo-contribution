use crate::backend::PacketMessage;
use crate::client::backend::ClientBackend;
use crate::client::frontend::ClientNetworkRequest::{Get, ListAll};
use crate::client::frontend::RequestWrapper;
use common_structs::message::Message::{ReqFile, ReqFilesList};
use common_structs::message::{Message, ServerType};
use common_structs::types::SessionId;
use wg_2024::network::NodeId;
use wg_2024::packet::NodeType;

impl ClientBackend {
    pub(super) fn handle_frontend_request(&mut self, rq: RequestWrapper) {
        let res = self.handle_frontend_rq_types(&rq);

        let Some(session) = res else {
            return rq.post_err_not_found();
        };

        self.open_requests.insert(session, rq);
    }

    fn handle_frontend_rq_types(&mut self, rq: &RequestWrapper) -> Option<SessionId> {
        let session_id = self.fresh_session_id();

        match rq.get_request()? {
            ListAll => {
                let mut count = 0;

                // TODO remove clone
                for (id, server_type) in self.servers.clone() {
                    if let Some(ServerType::Text) = server_type {
                        let part_session_id = self.fresh_session_id();
                        let packet_msg = PacketMessage::new(part_session_id, id, ReqFilesList);
                        self.split_req.insert(part_session_id, session_id);
                        let _ = self.network_send.send(packet_msg);

                        count += 1;
                    }
                }

                if count == 0 {
                    return None;
                }

                self.accumulator_list_all
                    .insert(session_id, (count, vec![]));
            }
            Get(link) => {
                let server_id = self.get_from_dns(&link)?;
                let packet_msg = PacketMessage::new(session_id, server_id, ReqFile(link));
                let _ = self.network_send.send(packet_msg);
            }
        }

        Some(session_id)
    }

    pub(super) fn handle_new_leaf(&mut self, node_id: NodeId, node_type: NodeType) {
        println!("{node_id} {node_type:?}");
        if node_type == NodeType::Server {
            println!("{node_id}");
            self.servers.push((node_id, None));

            let packet_req_type =
                PacketMessage::new(self.fresh_session_id(), node_id, Message::ReqServerType);

            let _ = self.network_send.send(packet_req_type);
        }
    }

    fn fresh_session_id(&mut self) -> u64 {
        let res = self.new_session_id;
        self.new_session_id += 1;
        res
    }
}
