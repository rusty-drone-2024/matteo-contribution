use common_structs::message::{FileWithData, Link, Media};
use serde::{Deserialize, Serialize};
use std::net::TcpStream;
use wg_2024::network::NodeId;

mod io;
pub mod send;

pub struct RequestWrapper {
    stream: TcpStream,
}

impl From<TcpStream> for RequestWrapper {
    fn from(stream: TcpStream) -> Self {
        Self { stream }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GuiRequest {
    ListAll,
    Get(Link),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GuiResponse {
    Err404,
    ListOfAll(Vec<(NodeId, Vec<Link>)>),
    GotFile(FileWithData),
    #[allow(dead_code)]
    GotMedia(Media),
}
