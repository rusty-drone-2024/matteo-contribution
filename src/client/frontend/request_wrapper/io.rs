use crate::client::frontend::request_wrapper::ClientNetworkRequest::{Get, ListAll};
use crate::client::frontend::request_wrapper::ClientNetworkResponse::{
    GotFile, GotMedia, ListOfAll,
};
use crate::client::frontend::request_wrapper::{
    ClientNetworkRequest, ClientNetworkResponse, RequestWrapper,
};
use crate::client::frontend::ClientNetworkResponse::Err404;
use tiny_http::Response;

impl RequestWrapper {
    pub fn get_request(&self) -> Option<ClientNetworkRequest> {
        let link = self.rq.url();

        if link.is_empty() || link == "/" {
            return Some(ListAll);
        } else if let Some(link) = link.strip_prefix("/file/") {
            return Some(Get(link.to_string()));
        }
        None
    }

    pub fn post_response(self, response: ClientNetworkResponse) -> Option<()> {
        let to_send = match response {
            ListOfAll(ref list) => Response::from_data(Self::create_homepage(list)),
            GotFile(file_with_media) => {
                let mut file = file_with_media.file;

                let base = get_host(self.rq.url().strip_prefix("/file/").unwrap_or_default());
                let base = base.unwrap_or_default();
                file = file.replace("url(/", &format!("url({base}/"));
                file = file.replace("href=\"/", &format!("href=\"{base}/"));
                file = file.replace("src=\"/", &format!("src=\"{base}/"));
                file = file.replace("action=\"/", &format!("action=\"{base}/"));
                file = file.replace("xmlns=\"/", &format!("xmnls=\"{base}/"));

                file = file.replace("url(", "url(/file/");
                file = file.replace("href=\"", "href=\"/file/");
                file = file.replace("src=\"", "src=\"/file/");
                file = file.replace("action=\"", "action=\"/file/");
                file = file.replace("xmnls=\"", "xmlns=\"/file/");

                file = file.replace("url(/file/\"", "url(\"");

                Response::from_data(file.to_string())
            }
            GotMedia(media) => Response::from_data(media),
            Err404 => {
                self.rq.respond(Response::empty(404)).ok();
                return None;
            }
        };

        self.rq.respond(to_send).ok()
    }

    pub fn post_err_not_found(self) {
        let _ = self.rq.respond(Response::empty(404));
    }
}

fn get_host(mut link: &str) -> Option<String> {
    let mut safe_protocol = false;
    if let Some(res) = link.strip_prefix("http://") {
        link = res;
    } else if let Some(res) = link.strip_prefix("https://") {
        safe_protocol = true;
        link = res;
    }

    let base = link.split('/').next()?;
    let protocol = if safe_protocol { "https" } else { "http" };
    Some(format!("{protocol}://{base}"))
}
