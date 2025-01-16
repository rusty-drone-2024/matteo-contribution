mod socket;

use crossbeam_channel::Sender;
use tiny_http::Request;
use wg_2024::network::NodeId;

pub struct FrontendWebServer {
    requests_channel: Sender<Request>,
    node_id: NodeId,
}

impl FrontendWebServer {
    pub fn new(node_id: NodeId, requests_channel: Sender<Request>) -> Self {
        Self {
            node_id,
            requests_channel,
        }
    }
}
