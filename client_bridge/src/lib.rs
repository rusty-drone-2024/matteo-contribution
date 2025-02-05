#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
use common_structs::message::{FileWithData, Link, Media};
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
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
    GetMedia(Link),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GuiResponse {
    Err404,
    ListOfAll(Vec<(NodeId, Vec<Link>)>),
    GotFile(FileWithData),
    GotMedia(Media),
}
