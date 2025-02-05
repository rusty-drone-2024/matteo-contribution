use crate::client::backend::ClientBackend;
use client_bridge::{GuiResponse, RequestWrapper};
use client_bridge::GuiResponse::{Err404, GotFile, GotMedia, ListOfAll};
use common_structs::message::{Link};
use common_structs::message::Message::{RespFile, RespFilesList, RespMedia, RespServerType, ErrNotFound};
use common_structs::types::Session;
use network::PacketMessage;
use wg_2024::network::NodeId;
use crate::client::backend::requests::RequestToNet::{Get, List, ListPartial};

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
                let pos = self.servers.iter().position(|&(id, _)| id == server_id)?;
                let server = self.servers.get_mut(pos)?;
                server.1 = Some(server_type);
                return None;
            }
            (RespFile(file), Some(Get{rq, link})) => {
                for (media, node) in &file.related_data {
                    self.save_to_dns(*node, media.clone());
                }

                rq.post_response(GotFile(link, file));
            }
            (RespMedia(media), Some(Get{rq, link})) => {
                rq.post_response(GotMedia(link, media));
            },
            (ErrNotFound, Some(Get{rq, ..})) => {
                // TODO Use better errors
                rq.post_response(Err404);
            },
            (RespFilesList(list), Some(ListPartial (main_session))) => {
                let (rq, resp) = self.handle_resp_files_list(main_session, server_id, list)?;
                rq.post_response(resp);
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
        list: Vec<Link>,
    ) -> Option<(RequestWrapper, GuiResponse)> {
        let net_req = self.open_requests.get_mut(&main_session)?;
        let List{ to_wait, acc, .. } = net_req else {
            return None;
        };

        acc.push((server_id, list.clone()));
        *to_wait -= 1;
        let is_finished = *to_wait == 0;

        for link in list {
            self.save_to_dns(server_id, link);
        }

        if is_finished {
            let net_req = self.open_requests.remove(&main_session)?;
            let List{ rq, acc, .. } = net_req else {
                return None;
            };
            Some((rq, ListOfAll(acc)))
        } else {
            None
        }
    }
}
