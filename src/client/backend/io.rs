use super::ClientBackend;
use crate::client::frontend::ClientNetworkRequest::{Get, ListAll};
use crate::client::frontend::ClientNetworkResponse::{GotFile, GotMedia, ListOfAll};
use crate::client::frontend::{ClientNetworkRequest, ClientNetworkResponse};
use common_structs::message::Message;
use common_structs::message::Message::{RespFile, RespFilesList, RespMedia};

impl ClientBackend {
    pub(super) fn convert_response(message: Message) -> Option<ClientNetworkResponse> {
        match message {
            RespFilesList(list) => Some(ListOfAll(list)),
            RespFile(file) => Some(GotFile(file)),
            RespMedia(media) => Some(GotMedia(media)),
            other => {
                // TODO handle ErrNotFound
                eprintln!("WARN message currently unsupported {other:?}");
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
