use super::ClientBackend;
use crate::client::frontend::ClientNetworkRequest::{Get, ListAll};
use crate::client::frontend::ClientNetworkResponse::{GotFile, GotMedia, ListOfAll};
use crate::client::frontend::{ClientNetworkRequest, ClientNetworkResponse};
use common_structs::message::Message;

impl ClientBackend {
    pub(super) fn convert_response(message: Message) -> Option<ClientNetworkResponse> {
        match message {
            Message::RespFilesList(list) => Some(ListOfAll(list)),
            Message::RespFile(file) => Some(GotFile(file)),
            Message::RespMedia(media) => Some(GotMedia(media)),
            _ => {
                // TODO handle ErrNotFound
                println!("WARN message currently unsupported");
                None
            }
        }
    }

    pub(super) fn convert_request(client_request: ClientNetworkRequest) -> Message {
        match client_request {
            ListAll => Message::ReqFilesList,
            Get(link) => Message::ReqFile(link),
        }
    }
}
