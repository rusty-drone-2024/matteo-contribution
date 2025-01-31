use crate::send::{recv_over, send_over};
use crate::{GuiRequest, GuiResponse, RequestWrapper};

impl RequestWrapper {
    pub fn take_request(&mut self) -> Option<GuiRequest> {
        recv_over(&mut self.stream)
    }

    pub fn post_err_not_found(self) -> Option<()> {
        self.post_response(GuiResponse::Err404)
    }

    pub fn post_response(mut self, response: GuiResponse) -> Option<()> {
        send_over(&mut self.stream, response.clone())
    }
}
