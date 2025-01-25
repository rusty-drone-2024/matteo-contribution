mod dns;
mod io;
mod request_handler;

use crate::backend::PacketMessage;
use crate::client::frontend::RequestWrapper;
use common_structs::message::Link;
use crossbeam_channel::{select, Receiver, Sender};
use std::collections::HashMap;
use wg_2024::network::NodeId;
use crate::backend::network::NetworkOutput;

pub struct ClientBackend {
    new_session_id: u64,
    open_requests: HashMap<u64, RequestWrapper>,
    dns: HashMap<Link, NodeId>,
    frontend_rcv: Receiver<RequestWrapper>,
    network_rcv: Receiver<NetworkOutput>,
    network_send: Sender<PacketMessage>,
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
            new_session_id: 0,
            open_requests: HashMap::default(),
            dns: HashMap::default(),
            frontend_rcv,
            network_rcv,
            network_send,
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
                        NetworkOutput::NewLeafFound(_,_) => {
                            todo!();
                        },
                    }
                },
            }
        }
    }
}
