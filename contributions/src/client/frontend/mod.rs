use client_bridge::RequestWrapper;
use crossbeam_channel::Sender;
use std::net::TcpListener;
use std::process::Command;
use wg_2024::network::NodeId;

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
        let Some((server, addr)) = self.init_server() else {
            return eprintln!("FATAL: Cannot initialize TCP server");
        };

        // TODO fix its use (+ is temp fix)
        let mut child = Command::new("cargo")
            .arg("run")
            .arg("--bin")
            .arg("client_ui")
            .arg(addr)
            .spawn()
            .unwrap();

        while let Ok((stream, _)) = server.accept() {
            let _ = self.requests_channel.send(stream.into());
        }

        child.wait().unwrap();
    }

    fn init_server(&self) -> Option<(TcpListener, String)> {
        let port = 7700 + i32::from(self.node_id);
        let addr = &format!("localhost:{port}");

        let server = TcpListener::bind(addr).ok()?;
        Some((server, addr.to_string()))
    }
}
