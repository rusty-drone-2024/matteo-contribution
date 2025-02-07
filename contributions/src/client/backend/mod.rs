mod dns;
mod input;
mod output;
mod requests;

use crate::client::backend::dns::Dns;
use crate::client::backend::requests::RequestToNet;
use client_bridge::RequestWrapper;
use common_structs::message::ServerType;
use common_structs::types::Session;
use crossbeam_channel::{select, Receiver, Sender};
use network::PacketMessage;
use network::{NetworkCommunication, NetworkOutput};
use std::collections::HashMap;
use tokio_util::sync::CancellationToken;
use wg_2024::network::NodeId;

pub struct ClientBackend {
    new_session: Session,
    open_requests: HashMap<Session, RequestToNet>,
    dns: Dns,
    servers: HashMap<NodeId, ServerType>,
    close_frontend_token: CancellationToken,
    frontend_rcv: Receiver<RequestWrapper>,
    net: NetworkCommunication,
}

impl ClientBackend {
    pub fn new(
        frontend_rcv: Receiver<RequestWrapper>,
        network_rcv: Receiver<NetworkOutput>,
        network_send: Sender<PacketMessage>,
        close_frontend_token: CancellationToken,
    ) -> Self
    where
        Self: Sized,
    {
        Self {
            new_session: 0,
            open_requests: HashMap::default(),
            dns: Dns::default(),
            servers: HashMap::default(),
            close_frontend_token,
            frontend_rcv,
            net: NetworkCommunication {
                backend: None,
                receiver: network_rcv,
                sender: network_send,
            },
        }
    }

    pub fn loop_forever(mut self) {
        loop {
            select! {
                recv(self.frontend_rcv) -> res => {
                    let Ok(frontend_rq) = res else {
                        break;
                    };
                    self.handle_frontend_request(frontend_rq);
                },
                recv(self.net.receiver) -> res => {
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

        self.close_frontend_token.cancel();
    }
}
