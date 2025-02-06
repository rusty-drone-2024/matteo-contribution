use crate::client::backend::requests::RequestToNet::{Get, List, ListPartial};
use crate::client::backend::ClientBackend;
use client_bridge::GuiResponse::{Err404, GotFile, GotMedia, ListOfAll};
use client_bridge::{GuiResponse, RequestWrapper};
use common_structs::message::Message::{
    ErrNotFound, RespFile, RespFilesList, RespMedia, RespServerType,
};
use common_structs::message::{Link, ServerType, ServerUUID};
use common_structs::types::Session;
use network::PacketMessage;
use wg_2024::network::NodeId;

impl ClientBackend {
    pub(super) fn handle_network_response(&mut self, packet_message: PacketMessage) -> Option<()> {
        let PacketMessage {
            session,
            message,
            opposite_end: server_id,
        } = packet_message;

        let net_req = self.open_requests.remove(&session);

        match (message, net_req) {
            (RespServerType(server_type), _) => {
                #[allow(clippy::match_same_arms)]
                match &server_type {
                    ServerType::Text(uuid) => self.dns.add_server(*uuid, server_id),
                    ServerType::Media(uuid) => self.dns.add_server(*uuid, server_id),
                    ServerType::Chat => {}
                }
                self.servers.insert(server_id, server_type)?;
                return None;
            }
            (RespFile(file), Some(Get { rq, link })) => {
                for (media_link, uuid) in &file.related_data {
                    self.dns.save(media_link.clone(), *uuid);
                }

                let _ = rq.post_response(GotFile(link, file));
            }
            (RespMedia(media), Some(Get { rq, link })) => {
                let _ = rq.post_response(GotMedia(link, media));
            }
            (ErrNotFound, Some(Get { rq, .. })) => {
                let _ = rq.post_response(Err404);
            }
            (RespFilesList(list), Some(ListPartial{session, uuid})) => {
                let (rq, resp) = self.handle_resp_files_list(session, server_id, uuid, list)?;
                let _ = rq.post_response(resp);
            }
            (_, _) => {
                return None;
            }
        };
        Some(())
    }

    fn handle_resp_files_list(
        &mut self,
        main_session: Session,
        server_id: NodeId,
        uuid: ServerUUID,
        list: Vec<Link>,
    ) -> Option<(RequestWrapper, GuiResponse)> {
        let net_req = self.open_requests.get_mut(&main_session)?;
        let List { to_wait, acc, .. } = net_req else {
            return None;
        };

        acc.push((server_id, list.clone()));
        *to_wait -= 1;
        let is_finished = *to_wait == 0;

        for link in list {
            self.dns.save(link, uuid);
        }

        if is_finished {
            let net_req = self.open_requests.remove(&main_session)?;
            let List { rq, acc, .. } = net_req else {
                return None;
            };
            Some((rq, ListOfAll(acc)))
        } else {
            None
        }
    }
}
