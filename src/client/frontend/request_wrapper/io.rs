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

        let link = link.strip_prefix("/file/").ok_or(())?.to_owned();
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
}
