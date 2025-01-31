use client_bridge::RequestWrapper;
use crossbeam_channel::Sender;
use std::env::current_exe;
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

        let mut exe = current_exe().unwrap();
        println!("{exe:?}");
        exe.pop();
        exe.push("client_ui");

        // TODO use it
        let a = Command::new(exe).arg(addr).spawn();
        println!("{a:?}");

        while let Ok((stream, _)) = server.accept() {
            // TODO handle
            let _ = self.requests_channel.send(stream.into());
        }
        dbg!("EXITING TCP");
    }

    fn init_server(&self) -> Option<(TcpListener, String)> {
        let port = 7700 + i32::from(self.node_id);
        let addr = &format!("localhost:{port}");
        let server = TcpListener::bind(addr).ok()?;

        println!("OPEN page http://{addr} for media client");
        Some((server, addr.to_string()))
    }
}
