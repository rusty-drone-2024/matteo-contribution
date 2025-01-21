use common_structs::message::{FileWithData, Link, Media};
use tiny_http::Request;

mod io;
mod pages;

pub struct RequestWrapper {
    rq: Request,
}

impl From<Request> for RequestWrapper {
    fn from(rq: Request) -> Self {
        Self { rq }
    }
}

#[derive(Debug, Clone)]
pub enum ClientNetworkRequest {
    ListAll,
    Get(Link),
}

#[derive(Debug, Clone)]
pub enum ClientNetworkResponse {
    ListOfAll(Vec<Link>),
    GotFile(FileWithData),
    #[allow(dead_code)]
    GotMedia(Media),
}