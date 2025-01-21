mod request_wrapper;

use crossbeam_channel::Sender;
use tiny_http::Server;
use wg_2024::network::NodeId;

pub use request_wrapper::*;

pub struct ClientFrontend {
    requests_channel: Sender<RequestWrapper>,
    node_id: NodeId,
}

impl ClientFrontend {
    pub fn new(node_id: NodeId, requests_channel: Sender<RequestWrapper>) -> Self {
        Self {
            requests_channel,
            node_id,
        }
    }

    pub fn loop_forever(&self) {
        let Some(server) = self.init_server() else {
            return eprintln!("FATAL: Cannot initialize web server");
        };

        while let Ok(rq) = server.recv() {
            let _ = self.requests_channel.send(RequestWrapper::from(rq));
        }
    }

    fn init_server(&self) -> Option<Server> {
        let port = 7700 + i32::from(self.node_id);
        let addr = &format!("localhost:{port}");
        let server = Server::http(addr).ok()?;

        println!("OPEN page http://{addr} for media client");
        Some(server)
    }
}
