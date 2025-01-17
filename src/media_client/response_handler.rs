use crate::backend::ClientNetworkResponse;
use crate::media_client::MediaClient;
use common_structs::message::Link;
use std::collections::HashMap;
use tiny_http::Response;
use wg_2024::network::NodeId;

impl MediaClient {
    pub(super) fn handle_response(&mut self, response: ClientNetworkResponse) {
        println!("NET RESPONSE {:?} {:?}", response, self.open_requests);

        match response {
            ClientNetworkResponse::ListOfAll(request_id, list) => {
                if let Some(request) = self.open_requests.remove(&request_id) {
                    let html = self.create_homepage(&list);
                    let _ = request.respond(Response::from_data(html));

                    for (link, node_id) in list {
                        self.dns.insert(link, node_id);
                    }
                }
            }
            ClientNetworkResponse::GotFile(request_id, file) => {
                if let Some(request) = self.open_requests.remove(&request_id) {
                    let _ = request.respond(Response::from_data(file.file));
                }
            }
            ClientNetworkResponse::GotMedia(_request_id, _link, _media) => {}
        }
    }

    fn create_homepage(&self, list: &HashMap<Link, NodeId>) -> String {
        let mut html = "<!DOCTYPE html><html><body><h1>Link possibles</h1>".to_string();
        for link in list.keys() {
            html.push_str(&format!("<a href=\".?link={}\">{}</a><br>", link, link));
        }

        html.push_str("</body></html>");
        html.to_string()
    }
}
