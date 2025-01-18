use crate::client::frontend::FrontendWebServer;
use tiny_http::Server;

impl FrontendWebServer {
    pub fn loop_forever(&self) {
        let Some(server) = self.init_server() else {
            return println!("FATAL: Cannot initialize web server");
        };

        while let Ok(rq) = server.recv(){
            let _ = self.requests_channel.send(rq);
        }

        println!("Channel returned in web frontend");
    }

    fn init_server(&self) -> Option<Server> {
        let port = 7700 + i32::from(self.node_id);
        let addr = &format!("localhost:{port}");
        let server = Server::http(addr).ok()?;

        println!("OPEN page http://{addr} for media client");
        Some(server)
    }
}
