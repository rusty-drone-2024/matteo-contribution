use crate::local_server::FrontendWebServer;
use crate::utils::set_panics_message;
use tiny_http::Server;

impl FrontendWebServer {
    pub fn loop_forever(&self) {
        set_panics_message("Failed webserver");
        let server = self.init_server();

        loop {
            match server.recv() {
                Ok(rq) => {
                    let _ = self.requests_channel.send(rq);
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            };
        }
    }

    fn init_server(&self) -> Server {
        let port = 7700 + self.node_id as u32;
        let addr = &format!("localhost:{}", port);
        let server = Server::http(addr).unwrap();
        println!("OPEN page http://{} for media client", addr);
        server
    }
}
