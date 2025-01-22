use crate::backend::PacketMessage;
use crate::client::backend::ClientBackend;
use crate::client::frontend::RequestWrapper;

impl ClientBackend {
    pub(super) fn handle_frontend_request(&mut self, frontend_request: RequestWrapper) {
        let Ok(client_req) = frontend_request.get_request() else {
            return frontend_request.post_err_not_found();
        };

        let session_id = self.fresh_session_id();
        self.open_requests.insert(session_id, frontend_request);

        //TODO remove unwrap
        let server_id = self.get_from_dns(&client_req).unwrap();
        let msg = Self::convert_request(client_req);

        let packet_msg = PacketMessage::new(session_id, server_id, msg);
        let _ = self.network_send.send(packet_msg);
    }

    fn fresh_session_id(&mut self) -> u64 {
        let res = self.new_session_id;
        self.new_session_id += 1;
        res
    }

    pub(super) fn handle_network_response(&mut self, packet_message: PacketMessage) -> Option<()> {
        let PacketMessage {
            session,
            message,
            opposite_end,
        } = packet_message;

        let resp = Self::convert_response(message)?;
        self.save_to_dns(opposite_end, &resp);
        let frontend_request = self.open_requests.remove(&session)?;
        frontend_request.post_response(resp)
    }
}
