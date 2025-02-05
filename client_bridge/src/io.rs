use crate::send::{recv_over, send_over};
use crate::{GuiRequest, GuiResponse, RequestWrapper};

impl RequestWrapper {
    pub fn take_request(&mut self) -> Option<GuiRequest> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .ok()?;

        rt.block_on(recv_over(&mut self.stream))
    }

    pub fn post_err_not_found(self) -> Option<()> {
        self.post_response(GuiResponse::Err404)
    }

    pub fn post_response(mut self, response: GuiResponse) -> Option<()> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .ok()?;

        rt.block_on(send_over(&mut self.stream, response))
    }
}
