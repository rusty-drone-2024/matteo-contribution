use crate::low_level::network_level::NetworkHandler;
use crate::low_level::{ClientNetworkRequest, ClientNetworkResponse};
use common_structs::message::{FileWithData, Message};
use std::collections::HashMap;
use wg_2024::network::SourceRoutingHeader;

impl NetworkHandler {
    pub(crate) fn handle_client_request(&mut self, client_request: ClientNetworkRequest) {
        match client_request {
            ClientNetworkRequest::ListAll(id) => {
                self.disassembler
                    .add_message_to_send(id, Message::RespFilesList(vec![12, 13]));
                for fragment_index in 0..self.disassembler.get_fragment_size(id).unwrap() {
                    self.packet_senders[&20]
                        .send(
                            self.disassembler
                                .get_packet_for_fragment(
                                    SourceRoutingHeader::initialize(vec![20, 10]),
                                    id,
                                    fragment_index,
                                )
                                .unwrap(),
                        )
                        .unwrap()
                }
            }
            ClientNetworkRequest::Get(id, node_id, link) => {
                let file = format!(
                    "<!DOCTYPE html><html><body><h1>{} - {}</h1></body></html>",
                    node_id, link
                );
                let _ = self.client_sender.send(ClientNetworkResponse::GotFile(
                    id,
                    FileWithData {
                        file,
                        related_data: HashMap::new(),
                    },
                ));
            }
        }
    }
}
