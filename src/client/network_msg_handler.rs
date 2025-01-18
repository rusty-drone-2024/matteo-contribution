use crate::backend::network::PacketMessage;
use crate::backend::ClientNetworkResponse;
use crate::backend::ClientNetworkResponse::{GotFile, GotMedia, ListOfAll};
use crate::client::TextMediaClientBackend;
use common_structs::message::Message;
use common_structs::types::SessionId;
use std::collections::HashMap;

impl TextMediaClientBackend {
    pub(crate) fn handle_message_packet(&mut self, packet_msg: PacketMessage) {
        let PacketMessage(session_id, _, message) = packet_msg;

        if let Some(resp) = self.trasform_into_client_response(session_id, message) {
            let _ = self.client_sender.send(resp);
        }
    }

    fn trasform_into_client_response(
        &self,
        session_id: SessionId,
        message: Message,
    ) -> Option<ClientNetworkResponse> {
        match message {
            Message::RespFilesList(list) => {
                let result = list
                    .into_iter()
                    .map(|x| (x.to_string(), 20u8))
                    .collect::<HashMap<_, _>>();
                Some(ListOfAll(session_id, result))
            }
            Message::RespFile(file) => Some(GotFile(session_id, file)),
            Message::RespMedia(media) => Some(GotMedia(session_id, "BOh".to_string(), media)),
            // TODO LET CLIENT HANDLE THAT Message::ErrNotFound => {}
            _ => {
                println!("WARN message currently unsupported");
                None
            }
        }
    }
}
