use crate::client::frontend::request_wrapper::ClientNetworkRequest::{Get, ListAll};
use crate::client::frontend::request_wrapper::ClientNetworkResponse::{
    GotFile, GotMedia, ListOfAll,
};
use crate::client::frontend::request_wrapper::{
    ClientNetworkRequest, ClientNetworkResponse, RequestWrapper,
};
use tiny_http::Response;

impl RequestWrapper {
    pub fn get_request(&self) -> Result<ClientNetworkRequest, ()> {
        let link = self.rq.url();

        if link.is_empty() || link == "/" {
            return Ok(ListAll);
        }

        let link = link.strip_prefix("/?link=").ok_or(())?.to_owned();
        Ok(Get(link))
    }

    pub fn post_response(self, response: ClientNetworkResponse) -> Option<()> {
        let to_send = match response {
            ListOfAll(ref list) => Response::from_data(Self::create_homepage(list)),
            GotFile(file_with_media) => Response::from_data(file_with_media.file),
            GotMedia(media) => Response::from_data(media),
        };

        self.rq.respond(to_send).ok()
    }

    pub fn post_err_not_found(self) {
        let _ = self.rq.respond(Response::empty(404));
    }

    /*
    pub fn get_request(&mut self, rq: Request) -> Option<ClientNetworkRequest>{
        println!("REQUESTING {}", rq.url());
        let to_forward = self.trasform_to_network_req(rq.url())?;
        println!("-- FORWARDING {to_forward:?}");

        TODO
        self.open_requests.insert(self.counter, rq);
        self.counter += 1;
        Some(to_forward)

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

    pub fn handle_response(&mut self, response: ClientNetworkResponse) {
        println!("NET RESPONSE {:?} {:?}", response, self.open_requests);

        match response {
            ClientNetworkResponse::ListOfAll(request_id, list) => {
                let Some(request) = self.open_requests.remove(&request_id) else {
                    return;
                };

                let html = Self::create_homepage(&list);
                let _ = request.respond(Response::from_data(html));

                for (link, node_id) in list {
                    self.dns.insert(link, node_id);
                }
            }
            ClientNetworkResponse::GotFile(request_id, file) => {
                let Some(request) = self.open_requests.remove(&request_id) else {
                    return;
                };

                let _ = request.respond(Response::from_data(file.file));
            }
            ClientNetworkResponse::GotMedia(_request_id, _link, _media) => {}
        }
    }

    fn create_homepage(list: &HashMap<Link, NodeId>) -> String {
        let mut html = "<!DOCTYPE html><html><body><h1>Link possibles</h1>".to_string();
        for link in list.keys() {
            html.push_str(&format!("<a href=\".?link={link}\">{link}</a><br>"));
        }

        html.push_str("</body></html>");
        html.to_string()
    }
    */
}
