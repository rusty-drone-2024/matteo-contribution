use crate::low_level::{ClientNetworkRequest, ClientNetworkResponse};

use common_structs::message::FileWithData;
use crossbeam_channel::{Receiver, Sender};
use std::collections::HashMap;

pub fn run_network_handler(
    receiver: Receiver<ClientNetworkRequest>,
    sender: Sender<ClientNetworkResponse>,
) {
    while let Ok(client_request) = receiver.recv() {
        match client_request {
            ClientNetworkRequest::ListAll(id) => {
                let list = vec![
                    ("https:://www.filebello.com".to_string(), 5),
                    ("https:://www.filebello2.com".to_string(), 3),
                    ("marco".to_string(), 2),
                ];
                let _ = sender.send(ClientNetworkResponse::ListOfAll(
                    id,
                    list.into_iter().collect(),
                ));
            }
            ClientNetworkRequest::Get(id, node_id, link) => {
                let file = format!(
                    "<!DOCTYPE html><html><body><h1>{} - {}</h1></body></html>",
                    node_id, link
                );
                let _ = sender.send(ClientNetworkResponse::GotFile(
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
