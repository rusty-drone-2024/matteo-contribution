use crate::backend::NetworkBackend;
use common_structs::types::{FragmentIdx, Routing, Session};
use wg_2024::network::NodeId;
use wg_2024::packet::{FloodRequest, FloodResponse, Nack, NackType, NodeType, Packet};

impl NetworkBackend {
    pub(super) fn new_ack(
        mut routing: Routing,
        session: Session,
        fragment_id: FragmentIdx,
    ) -> Packet {
        routing.reverse();
        routing.increase_hop_index();

        Packet::new_ack(routing, session, fragment_id)
    }

    pub(super) fn new_nack(
        self_id: NodeId,
        routing: Routing,
        session: Session,
        fragment_id: FragmentIdx,
        nack_type: NackType,
    ) -> Packet {
        let hops = routing
            .hops
            .into_iter()
            .take(routing.hop_index)
            .chain(Some(self_id))
            .rev()
            .collect();

        let nack = Nack {
            fragment_index: fragment_id,
            nack_type,
        };

        let routing = Routing::new(hops, 1);
        Packet::new_nack(routing, session, nack)
    }

    pub(super) fn new_flood_resp(
        self_id: NodeId,
        self_type: NodeType,
        session: Session,
        flood: FloodRequest,
    ) -> Packet {
        let flood_id = flood.flood_id;
        let mut path_trace = flood.path_trace;

        path_trace.push((self_id, self_type));
        let hops = path_trace.iter().map(|(id, _)| *id).rev().collect();

        Packet::new_flood_response(
            Routing::with_first_hop(hops),
            session,
            FloodResponse {
                flood_id,
                path_trace,
            },
        )
    }
}
