mod socket;

use crossbeam_channel::Sender;
use tiny_http::Request;

pub struct FrontendWebServer {
    requests_channel: Sender<Request>,
}
