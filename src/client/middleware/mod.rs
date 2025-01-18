mod request_handler;
mod response_handler;

use crate::backend::{ClientNetworkRequest, ClientNetworkResponse};
use crate::client::frontend::FrontendWebServer;
use crate::client::TextMediaClientBackend;
use crate::utils::set_panics_message;
use common_structs::leaf::{Leaf, LeafCommand, LeafEvent};
use common_structs::message::Link;
use crossbeam_channel::{select, unbounded, Receiver, Sender};
use std::collections::HashMap;
use std::thread;
use tiny_http::Request;
use wg_2024::network::NodeId;
use wg_2024::packet::Packet;

pub struct TextMediaClient {
    webserver_requests: Receiver<Request>,
    open_requests: HashMap<u64, Request>,
    counter: u64,
    dns: HashMap<Link, NodeId>,
    network_request: Sender<ClientNetworkRequest>,
    network_response: Receiver<ClientNetworkResponse>,
}

impl Leaf for TextMediaClient {
    fn new(
        id: NodeId,
        _controller_send: Sender<LeafEvent>,
        _controller_recv: Receiver<LeafCommand>,
        packet_recv: Receiver<Packet>,
        packet_send: HashMap<NodeId, Sender<Packet>>,
    ) -> Self
    where
        Self: Sized,
    {
        let (web_requests_channel, webserver_requests) = unbounded();
        let (network_request, network_request_listener) = unbounded();
        let (network_response_sender, network_response) = unbounded();

        start_webserver(id, web_requests_channel);
        start_network_handler(
            network_request_listener,
            network_response_sender,
            packet_recv,
            packet_send,
        );

        Self {
            network_request,
            network_response,
            webserver_requests,
            counter: 0,
            open_requests: HashMap::new(),
            dns: HashMap::default(),
        }
    }

    fn run(&mut self) {
        set_panics_message("Failed client middleware");
        loop {
            select! {
                recv(self.webserver_requests) -> msg => {
                    self.handle_request(msg.unwrap());
                },
                recv(self.network_response) -> msg => {
                    self.handle_response(msg.unwrap());
                }
            }
        }
    }
}

fn start_webserver(node_id: NodeId, requests_channel: Sender<Request>) {
    let server = FrontendWebServer::new(node_id, requests_channel);

    thread::spawn(move || {
        server.loop_forever();
    });
}

fn start_network_handler(
    receiver: Receiver<ClientNetworkRequest>,
    sender: Sender<ClientNetworkResponse>,
    packet_receiver: Receiver<Packet>,
    packet_senders: HashMap<NodeId, Sender<Packet>>,
) {
    thread::spawn(move || {
        TextMediaClientBackend::new(
            packet_receiver,
            packet_senders.into_iter().next().unwrap().1,
            receiver,
            sender,
        )
        .run();
    });
}
