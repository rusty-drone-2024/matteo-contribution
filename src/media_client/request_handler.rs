use crate::low_level::network_handler::ClientNetworkRequest;
use crate::media_client::MediaClient;
use common_structs::message::Link;
use tiny_http::{Method, Request, Response};
use wg_2024::network::NodeId;

impl MediaClient {
    pub(super) fn handle_request(&mut self, rq: Request) {
        if *rq.method() != Method::Get {
            return;
        }

        println!("{}", rq.url());

        if rq.url().is_empty() || rq.url() == "/" {
            self.fetch_homepage_data();
        } else if rq.url().starts_with("/?link=") {
            let internal_url = &rq.url()[7..];

            if let Some(node_id) = self.dns.get(internal_url) {
                let link = internal_url.to_string();
                self.fetch_page_data(*node_id, link);
            } else {
                let _ = rq.respond(Response::empty(404));
                return;
            }
        } else {
            let _ = rq.respond(Response::empty(404));
            return;
        }
        

        self.open_requests.insert(self.counter, rq);
        self.counter += 1;
    }

    fn fetch_homepage_data(&self) {
        let _ = self
            .network_request
            .send(ClientNetworkRequest::ListAll(self.counter));
        println!("  --> ASK ALL");
    }

    fn fetch_page_data(&self, node_id: NodeId, url: Link) {
        let _ = self.network_request.send(ClientNetworkRequest::Get(
            self.counter,
            node_id,
            url.clone(),
        ));
        println!("  --> ASK {}", url);
    }
}
