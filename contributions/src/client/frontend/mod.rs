use client_bridge::RequestWrapper;
use crossbeam_channel::Sender;
use std::process::{Child, Command};
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;
use wg_2024::network::NodeId;

pub struct ClientFrontend {
    requests_channel: Sender<RequestWrapper>,
    close_req: CancellationToken,
    node_id: NodeId,
}

impl ClientFrontend {
    pub fn new(
        node_id: NodeId,
        requests_channel: Sender<RequestWrapper>,
        close_req: CancellationToken,
    ) -> Self {
        Self {
            requests_channel,
            close_req,
            node_id,
        }
    }

    pub fn loop_forever(&self) {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        rt.block_on(self.handle_requests());
    }

    async fn handle_requests(&self) {
        let Some((server, addr)) = self.init_server().await else {
            return eprintln!("FATAL: Cannot initialize TCP server");
        };

        #[allow(clippy::suspicious)]
        let mut child = Self::run_gui(&addr);

        loop {
            tokio::select! {
                () = self.close_req.cancelled() => {
                    break;
                }
                Ok((stream, _)) = server.accept() => {
                    let _ = self.requests_channel.send(stream.into());
                }
                else => {
                    eprintln!("Error during server accepting");
                    break;
                }
            }
        }

        child.wait().unwrap();
    }

    async fn init_server(&self) -> Option<(TcpListener, String)> {
        let port = 7700 + i32::from(self.node_id);
        let addr = format!("localhost:{port}");

        let server = TcpListener::bind(&addr).await.ok()?;
        Some((server, addr))
    }

    fn run_gui(addr: &str) -> Child {
        // TODO fix its use (+ is temp fix)

        Command::new("cargo")
            .arg("run")
            .arg("--bin")
            .arg("client_ui")
            .arg(addr)
            .spawn()
            .unwrap()
    }
}
