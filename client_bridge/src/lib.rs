#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::cargo_common_metadata)]
//! Crate use to bridge communication between client frontend and backend

use common_structs::message::{FileWithData, Link, Media};
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use wg_2024::network::NodeId;

mod io;
pub mod send;

/// Wrapper to abstract over implementation details
/// Allow some simple operation such as getting the request
/// and respond with an error or some value
pub struct RequestWrapper {
    stream: TcpStream,
}

impl From<TcpStream> for RequestWrapper {
    fn from(stream: TcpStream) -> Self {
        Self { stream }
    }
}

/// Represent a request from the frontend to the backend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GuiRequest {
    /// Request all files of all servers
    ListAll,
    /// Request a file by link
    Get(Link),
    /// Request a media by link
    GetMedia(Link),
}

/// Represent a response from the backend to the frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GuiResponse {
    /// Generic error in case of it was not possible to obtain resource
    Err404,
    /// A list of all text server and their files
    ListOfAll(Vec<(NodeId, Vec<Link>)>),
    /// Got the files by the link with its content
    GotFile(Link, FileWithData),
    /// Got the media by the link with its content
    GotMedia(Link, Media),
}
