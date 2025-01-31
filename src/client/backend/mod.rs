mod dns;
mod input;
mod output;

use crate::backend::network::NetworkOutput;
use crate::backend::PacketMessage;
use client_bridge::RequestWrapper;
use common_structs::message::{Link, ServerType};
use common_structs::types::Session;
use crossbeam_channel::{select, Receiver, Sender};
use std::collections::HashMap;
use wg_2024::network::NodeId;

pub struct ClientBackend {
    new_session: u64,
    open_requests: HashMap<u64, RequestWrapper>,
    dns: HashMap<Link, NodeId>,
    frontend_rcv: Receiver<RequestWrapper>,
    network_rcv: Receiver<NetworkOutput>,
    network_send: Sender<PacketMessage>,
    servers: Vec<(NodeId, Option<ServerType>)>,
    /// Contains partial and total
    split_req: HashMap<Session, Session>,
    #[allow(clippy::type_complexity)]
    accumulator_list_all: HashMap<Session, (usize, Vec<(NodeId, Vec<Link>)>)>,
}

impl ClientBackend {
    pub fn new(
        frontend_rcv: Receiver<RequestWrapper>,
        network_rcv: Receiver<NetworkOutput>,
        network_send: Sender<PacketMessage>,
    ) -> Self
    where
        Self: Sized,
    {
        Self {
            new_session: 0,
            open_requests: HashMap::default(),
            dns: HashMap::default(),
            frontend_rcv,
            network_rcv,
            network_send,
            servers: vec![],
            split_req: HashMap::default(),
            accumulator_list_all: HashMap::default(),
        }
    }

    pub fn run(&mut self) {
        loop {
            select! {
                recv(self.frontend_rcv) -> res => {
                    let Ok(frontend_rq) = res else {
                        break;
                    };
                    self.handle_frontend_request(frontend_rq);
                },
                recv(self.network_rcv) -> res => {
                    let Ok(net_msg) = res else {
                        break;
                    };

                    match net_msg {
                        NetworkOutput::MsgReceived(msg) => {
                            self.handle_network_response(msg);
                        },
                        NetworkOutput::NewLeafFound(node_id, node_type) => {
                            self.handle_new_leaf(node_id, node_type);
                        },
                    }
                },
            }
        }
    }
}
