use std::env::current_exe;
use client_bridge::RequestWrapper;
use crossbeam_channel::Sender;
use std::process::{Child, Command};
use tokio::io;
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;
use wg_2024::network::NodeId;

/// Frontend message handler part of the client.
pub struct ClientFrontend {
    requests_channel: Sender<RequestWrapper>,
    close_req: CancellationToken,
    node_id: NodeId,
}

impl ClientFrontend {
    pub const fn new(
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

    /// Run the server forever on that thread.
    /// It only exit once the Kill signal as been received by the backend.
    pub fn loop_forever(self) {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        rt.block_on(self.server_functionality());
    }
}

impl ClientFrontend {
    /// Setup, run gui and handle requst until it terminate.
    async fn server_functionality(&self) {
        let Ok((server, addr)) = self.init_server().await else {
            return eprintln!("FATAL: Cannot initialize TCP server");
        };

        let mut child = Self::run_gui(&addr);
        self.handle_requests(server).await;

        let _ = child.kill();
        child.wait().unwrap();
    }

    /// Accept incoming request and handle them up until
    /// it either fail or it is closed becuase the backend
    /// received a `Kill` message.
    async fn handle_requests(&self, server: TcpListener) {
        loop {
            tokio::select! {
                () = self.close_req.cancelled() => {
                    return;
                }
                Ok((stream, _)) = server.accept() => {
                    let _ = self.requests_channel.send(stream.into());
                }
                else => {
                    eprintln!("Error during server accepting");
                    return;
                }
            }
        }
    }

    /// Initialize the server.
    /// # Return
    /// On success it return the `TcpListener` and the address used
    /// to initialize the server.
    /// # Errors
    /// It can fail if `TcpListener::bind` fails and in this case it
    /// return that error.
    async fn init_server(&self) -> io::Result<(TcpListener, String)> {
        let port = 7700 + i32::from(self.node_id);
        let addr = format!("localhost:{port}");

        let server = TcpListener::bind(&addr).await?;
        Ok((server, addr))
    }

    /// Execute the GUI
    /// To do so it uses cargo in debug mode as temp fix.
    /// In release mode it assume that the file is in `.resources/client_ui`
    fn run_gui(addr: &str) -> Child {
        let exe = current_exe();

        Command::new(exe.expect("FAIL").to_str().unwrap())
            .arg("media-gui")
            .arg(addr)
            .spawn()
            .expect("Couldn't open the required binary")
    }
}
