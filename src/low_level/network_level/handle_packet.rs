use crate::low_level::network_level::NetworkHandler;
use crate::low_level::ClientNetworkResponse::{GotFile, GotMedia, ListOfAll};
use common_structs::message::Message;
use common_structs::types::SessionId;
use std::collections::HashMap;
use wg_2024::packet::{Fragment, Packet, PacketType};

impl NetworkHandler {
    pub(crate) fn handle_packet(&mut self, packet: Packet) {
        let session_id = packet.session_id;

        match packet.pack_type {
            PacketType::MsgFragment(fragment) => self.handle_fragment(session_id, fragment),
            PacketType::Ack(_) => {}
            PacketType::Nack(_) => {}
            PacketType::FloodRequest(_) => {}
            PacketType::FloodResponse(_) => {}
        }
    }

    fn handle_fragment(&mut self, session_id: SessionId, fragment: Fragment) {
        self.assembler.add_fragment(session_id, fragment);
        if let Some(message) = self.assembler.get_full_message(session_id) {
            match message {
                //TODO it is just a start
                Message::RespFilesList(list) => {
                    let result = list
                        .into_iter()
                        .map(|x| (format!("{}", x), 20u8))
                        .collect::<HashMap<_, _>>();
                    self.client_sender
                        .send(ListOfAll(session_id, result))
                        .unwrap();
                }
                Message::RespFile(file) => {
                    self.client_sender.send(GotFile(1, file)).unwrap();
                }
                Message::RespMedia(media) => {
                    self.client_sender
                        .send(GotMedia(1, "BOh".to_string(), media))
                        .unwrap();
                }
                Message::ErrNotFound => {}
                //Message::RespClientList(_) => {}
                //Message::RespServerType(_) => {}
                _ => {
                    println!("WARN message currently unsupported");
                }
            }
        }
    }
}
