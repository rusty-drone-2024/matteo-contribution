use crate::client::backend::requests::RequestToNet;
use crate::client::backend::requests::RequestToNet::{Get, List, ListPartial};
use crate::client::backend::ClientBackend;
use client_bridge::{GuiRequest, RequestWrapper};
use common_structs::message::Message::{ReqFile, ReqFilesList, ReqMedia};
use common_structs::message::{Message, ServerType};
use common_structs::types::Session;
use network::PacketMessage;
use wg_2024::network::NodeId;
use wg_2024::packet::NodeType;

impl ClientBackend {
    pub(super) fn handle_frontend_request(&mut self, rq: RequestWrapper) {
        let session = self.fresh_session();
        let res = self.handle_frontend_rq_types(session, rq);

        match res {
            Ok(net_req) => {
                self.open_requests.insert(session, net_req);
            }
            Err(rq) => {
                let _ = rq.post_err_not_found();
            }
        };
    }

    fn handle_frontend_rq_types(
        &mut self,
        session: Session,
        mut rq: RequestWrapper,
    ) -> Result<RequestToNet, RequestWrapper> {
        let Ok(request) = rq.take_request() else {
            return Err(rq);
        };

        match request {
            GuiRequest::ListAll => {
                let mut to_wait = 0;

                for (id, server_type) in self.servers.clone() {
                    if let ServerType::Text(uuid) = server_type {
                        let part_session_id = self.fresh_session();
                        let packet_msg = PacketMessage::new(part_session_id, id, ReqFilesList);
                        self.open_requests
                            .insert(part_session_id, ListPartial { session, uuid });
                        let _ = self.net.sender.send(packet_msg);

                        to_wait += 1;
                    }
                }

                if to_wait == 0 {
                    // TODO Maybe send another message
                    return Err(rq);
                }

                Ok(List {
                    rq,
                    to_wait,
                    acc: vec![],
                })
            }
            GuiRequest::Get(link) => {
                let Some(server_id) = self.dns.get(&link) else {
                    return Err(rq);
                };

                let packet_msg = PacketMessage::new(session, server_id, ReqFile(link.clone()));
                let _ = self.net.sender.send(packet_msg);
                Ok(Get { rq, link })
            }
            GuiRequest::GetMedia(link) => {
                let Some(server_id) = self.dns.get(&link) else {
                    return Err(rq);
                };

                let packet_msg = PacketMessage::new(session, server_id, ReqMedia(link.clone()));
                let _ = self.net.sender.send(packet_msg);
                Ok(Get { rq, link })
            }
        }
    }

    pub(super) fn handle_new_leaf(&mut self, node_id: NodeId, node_type: NodeType) {
        if node_type == NodeType::Server {
            let packet_req_type =
                PacketMessage::new(self.fresh_session(), node_id, Message::ReqServerType);

            let _ = self.net.sender.send(packet_req_type);
        }
    }

    fn fresh_session(&mut self) -> u64 {
        let res = self.new_session;
        self.new_session += 1;
        res
    }
}
