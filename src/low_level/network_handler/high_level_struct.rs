use std::collections::HashMap;
use wg_2024::network::NodeId;
use common_structs::message::{FileWithData, Link, Media};

#[derive(Debug, Clone)]
pub enum ClientNetworkRequest{
    ListAll(u64),
    Get(u64, NodeId, Link),
}

#[derive(Debug, Clone)]
pub enum ClientNetworkResponse {
    ListOfAll(u64, HashMap<Link, NodeId>),
    GotFile(u64, FileWithData),
    GotMedia(u64, Link, Media),
}
