use crate::local_server::FrontendWebServer;
use crossbeam_channel::Sender;
use tiny_http::{Request, Server};

impl FrontendWebServer {
    pub fn new(requests_channel: Sender<Request>) -> Self {
        Self { requests_channel }
    }

    pub fn loop_forever(&self) {
        let server = Server::http("localhost:7878").unwrap();

        loop {
            match server.recv() {
                Ok(rq) => {
                    let _ = self.requests_channel.send(rq);
                }
                Err(e) => {
                    println!("error: {}", e);
                    break;
                }
            };
        }
    }
}
