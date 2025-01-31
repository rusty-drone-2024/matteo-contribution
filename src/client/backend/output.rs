use crate::backend::PacketMessage;
use crate::client::backend::ClientBackend;
use client_bridge::GuiResponse;
use client_bridge::GuiResponse::{Err404, GotFile, GotMedia, ListOfAll};
use common_structs::message::{Link, Message};
use common_structs::types::Session;
use wg_2024::network::NodeId;

impl ClientBackend {
    pub(super) fn handle_network_response(&mut self, packet_message: PacketMessage) -> Option<()> {
        let PacketMessage {
            mut session,
            message,
            opposite_end: server_id,
        } = packet_message;

        let resp = match message {
            Message::RespServerType(server_type) => {
                let pos = self.servers.iter().position(|&(id, _)| id == server_id)?;
                let server = self.servers.get_mut(pos)?;
                server.1 = Some(server_type);
                return None;
            }
            Message::RespFile(file) => {
                self.save_to_dns(server_id, file.file.clone());
                GotFile(file)
            }
            Message::RespMedia(media) => GotMedia(media),
            Message::ErrNotFound => Err404,
            Message::RespFilesList(list) => {
                let (id, response) = self.handle_resp_files_list(session, server_id, list)?;
                session = id;
                response
            }
            other => {
                eprintln!("WARN message currently unsupported {other:?}");
                return None;
            }
        };

        let frontend_request = self.open_requests.remove(&session)?;
        frontend_request.post_response(resp)
    }

    fn handle_resp_files_list(
        &mut self,
        session: Session,
        server_id: NodeId,
        list: Vec<Link>,
    ) -> Option<(Session, GuiResponse)> {
        let full_req_session = *self.split_req.get(&session)?;
        let acc = self.accumulator_list_all.get_mut(&full_req_session)?;

        acc.1.push((server_id, list.clone()));
        let finished = acc.0 == acc.1.len();

        for link in list {
            self.save_to_dns(server_id, link);
        }

        if !finished {
            return None;
        }

        self.split_req.remove(&session);
        let acc_res = self.accumulator_list_all.remove(&full_req_session)?.1;
        Some((full_req_session, ListOfAll(acc_res)))
    }
}
