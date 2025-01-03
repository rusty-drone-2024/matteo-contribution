mod request_handler;
mod response_handler;

use crate::local_server::FrontendWebServer;
use common_structs::leaf::{Leaf, LeafCommand, LeafPacketSentEvent};
use crossbeam_channel::{select, unbounded, Receiver, Sender};
use std::collections::HashMap;
use std::thread;
use tiny_http::Request;
use wg_2024::network::NodeId;
use wg_2024::packet::Packet;
use common_structs::message::Link;
use crate::low_level::network_handler::{run_network_handler, ClientNetworkRequest, ClientNetworkResponse};

pub struct MediaClient {
    webserver_requests: Receiver<Request>,
    open_requests: HashMap<u64, Request>,
    counter: u64,
    dns: HashMap<Link, NodeId>,
    network_request: Sender<ClientNetworkRequest>,
    network_response: Receiver<ClientNetworkResponse>,
}


impl MediaClient {
    pub fn new(
        _id: NodeId,
        _controller_send: Sender<LeafPacketSentEvent>,
        _controller_recv: Receiver<LeafCommand>,
        _packet_recv: Receiver<Packet>,
        _packet_send: HashMap<NodeId, Sender<Packet>>,
    ) -> Self
    where
        Self: Sized,
    {
        let (web_requests_channel, webserver_requests) = unbounded();
        let (network_request, network_request_listener) = unbounded();
        let (network_response_sender, network_response) = unbounded();

        start_webserver(web_requests_channel);
        start_network_handler(network_request_listener, network_response_sender);
        
        Self {
            initialized: false,
            network_request,
            network_response,
            webserver_requests,
            counter: 0,
            open_requests: HashMap::new(),
            dns: HashMap::default(),
        }
    }

    pub fn run(&mut self) {
        loop{
            select! {
                recv(self.webserver_requests) -> msg => {
                    println!("WEB REQUEST {:?}", msg);
                    self.handle_request(msg.unwrap());
                },
                recv(self.network_response) -> msg => {
                    println!("NET RESPONSE {:?}", msg);
                    self.handle_response(msg.unwrap());
                }
            }   
        }
    }
}

fn start_webserver(requests_channel: Sender<Request>) {
    let server = FrontendWebServer::new(requests_channel);

    thread::spawn(move || {
        server.loop_forever();
    });
}

fn start_network_handler(receiver: Receiver<ClientNetworkRequest>, sender: Sender<ClientNetworkResponse>) {
    thread::spawn(move || {
        run_network_handler(receiver, sender);
    });
}