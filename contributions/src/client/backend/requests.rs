use client_bridge::RequestWrapper;
use common_structs::message::{Link, ServerUUID};
use common_structs::types::Session;
use wg_2024::network::NodeId;

pub enum RequestToNet {
    List {
        rq: RequestWrapper,
        to_wait: usize,
        acc: Vec<(NodeId, Vec<Link>)>,
    },
    Get {
        rq: RequestWrapper,
        link: Link,
    },
    ListPartial{
        session: Session,
        uuid: ServerUUID,
    },
}
