use crate::backend::ClientNetworkRequest;
use crate::backend::ClientNetworkRequest::{Get, ListAll};
use crate::media_client::MediaClient;
use tiny_http::Request;

impl MediaClient {
    pub(super) fn handle_request(&mut self, rq: Request) {
        println!("REQUESTING {}", rq.url());
        let res_to_forward = self.trasform_to_network_req(rq.url());
        println!("-- FORWARDING {:?}", res_to_forward);

        if let Some(to_forward) = res_to_forward {
            self.open_requests.insert(self.counter, rq);
            self.counter += 1;
            let _ = self.network_request.send(to_forward);
        }
    }

    fn trasform_to_network_req(&self, link: &str) -> Option<ClientNetworkRequest> {
        if link.is_empty() || link == "/" {
            return Some(ListAll(self.counter));
        } else if let Some(link) = link.strip_prefix("/?link=") {
            let node_id = *self.dns.get(link)?;
            return Some(Get(self.counter, node_id, link.to_owned()));
        }
        None
    }
}
