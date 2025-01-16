use crate::low_level::network_handler::NetworkHandler;
use crate::low_level::{ClientNetworkRequest, ClientNetworkResponse};
use common_structs::message::FileWithData;
use std::collections::HashMap;

impl NetworkHandler {
    pub(super) fn handle_client_request(&self, client_request: ClientNetworkRequest) {
        match client_request {
            ClientNetworkRequest::ListAll(id) => {
                let list = vec![
                    ("https:://www.filebello.com".to_string(), 5),
                    ("https:://www.filebello2.com".to_string(), 3),
                    ("marco".to_string(), 2),
                ];
                let _ = self.client_sender.send(ClientNetworkResponse::ListOfAll(
                    id,
                    list.into_iter().collect(),
                ));
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
