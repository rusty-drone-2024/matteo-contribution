use crate::client::backend::ClientBackend;
use crate::client::frontend::ClientNetworkResponse::{GotFile, ListOfAll};
use crate::client::frontend::{ClientNetworkRequest, ClientNetworkResponse};
use wg_2024::network::NodeId;

impl ClientBackend {
    pub(super) fn save_to_dns(&mut self, node_id: NodeId, response: &ClientNetworkResponse) {
        match response {
            ListOfAll(list) => {
                for link in list {
                    // TODO use real value
                    self.dns.insert(link.clone(), node_id);
                }
            }
            GotFile(file) => {
                for (link, id) in &file.related_data {
                    self.dns.insert(link.clone(), *id);
                }
            }
            _ => {}
        }
    }

    pub(super) fn get_from_dns(&self, request: &ClientNetworkRequest) -> Option<NodeId> {
        match request {
            ClientNetworkRequest::ListAll => {
                //TODO remove hardcoded
                Some(20)
            }
            ClientNetworkRequest::Get(link) => self.dns.get(link).copied(),
        }
    }
}
