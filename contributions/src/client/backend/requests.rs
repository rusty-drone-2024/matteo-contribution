use common_structs::message::Link;
use common_structs::types::Session;
use wg_2024::network::NodeId;
use client_bridge::RequestWrapper;

pub enum RequestToNet{
    List{
        rq: RequestWrapper,
        to_wait: usize,
        acc: Vec<(NodeId, Vec<Link>)>
    },
    Get{
        rq: RequestWrapper,
        link: Link,
    },
    ListPartial(Session)
}