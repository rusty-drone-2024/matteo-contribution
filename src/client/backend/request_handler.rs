use crate::backend::PacketMessage;
use crate::client::backend::ClientBackend;
use crate::client::frontend::ClientNetworkResponse::ListOfAll;
use crate::client::frontend::{ClientNetworkResponse, RequestWrapper};

impl ClientBackend {
    pub(super) fn handle_frontend_request(&mut self, frontend_request: RequestWrapper) {
        let Ok(client_req) = frontend_request.get_request() else {
            return frontend_request.post_err_not_found();
        };

        let session_id = self.fresh_session_id();
        self.open_requests.insert(session_id, frontend_request);

        let msg = Self::convert_request(client_req);
        //TODO dehardcode
        let packet_msg = PacketMessage::new(session_id, 20, msg);
        let _ = self.network_send.send(packet_msg);
    }

    fn fresh_session_id(&mut self) -> u64 {
        let res = self.new_session_id;
        self.new_session_id += 1;
        res
    }

    pub(super) fn handle_network_response(&mut self, packet_message: PacketMessage) -> Option<()> {
        let PacketMessage {
            session, message, ..
        } = packet_message;

        let resp = Self::convert_response(message)?;
        self.save_to_dns_if_needed(&resp);
        let frontend_request = self.open_requests.remove(&session)?;
        frontend_request.post_response(resp)
    }

    fn save_to_dns_if_needed(&mut self, response: &ClientNetworkResponse) {
        let ListOfAll(list) = response else {
            return;
        };

        for link in list {
            // TODO use real value
            self.dns.insert(link.clone(), 20);
        }
    }
}
