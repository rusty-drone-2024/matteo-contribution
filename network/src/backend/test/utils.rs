use crate::backend::test::Net;
use common_structs::types::Routing;
use wg_2024::network::NodeId;
use wg_2024::packet::{FloodRequest, FloodResponse, NodeType, Packet};

const fn test_flood(source: NodeId, path_trace: Vec<(NodeId, NodeType)>) -> FloodRequest {
    FloodRequest {
        flood_id: 0,
        initiator_id: source,
        path_trace,
    }
}

fn test_flood_resp(path: Vec<(NodeId, NodeType)>) -> Packet {
    let hops = path.iter().map(|(id, _)| id).copied().rev().collect();

    Packet::new_flood_response(
        Routing::new(hops, 1),
        11,
        FloodResponse {
            flood_id: 0,
            path_trace: path,
        },
    )
}

#[test]
fn new_flood_resp_client() {
    let self_def = (20, NodeType::Client);
    let path = vec![(5, NodeType::Server), (10, NodeType::Drone)];
    let flood = test_flood(5, path);
    let res = Net::new_flood_resp(self_def.0, self_def.1, 11, flood);

    let expect_path = vec![(5, NodeType::Server), (10, NodeType::Drone), self_def];
    let expected = test_flood_resp(expect_path);
    assert_eq!(expected, res);
}

#[test]
fn new_flood_resp_server() {
    let self_def = (20, NodeType::Server);
    let path = vec![(5, NodeType::Server), (10, NodeType::Drone)];
    let flood = test_flood(5, path);
    let res = Net::new_flood_resp(self_def.0, self_def.1, 11, flood);

    let expect_path = vec![(5, NodeType::Server), (10, NodeType::Drone), self_def];
    let expected = test_flood_resp(expect_path);
    assert_eq!(expected, res);
}
