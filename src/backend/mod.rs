mod client;
pub mod network;
mod server;

pub use client::TextMediaClientBackend;
use common_structs::message::{FileWithData, Link, Media};
pub use server::TextServer;
use std::collections::HashMap;
use wg_2024::network::NodeId;

#[derive(Debug, Clone)]
pub enum ClientNetworkRequest {
    ListAll(u64),
    Get(u64, NodeId, Link),
}

#[derive(Debug, Clone)]
pub enum ClientNetworkResponse {
    ListOfAll(u64, HashMap<Link, NodeId>),
    GotFile(u64, FileWithData),
    #[allow(dead_code)]
    GotMedia(u64, Link, Media),
}