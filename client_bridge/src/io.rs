use crate::send::{recv_over, send_over, BridgeRecvError, BridgeSendError};
use crate::{GuiRequest, GuiResponse, RequestWrapper};

impl RequestWrapper {
    /// Get the request from the underlaying implementation
    /// It is a blocking call.
    /// # Return
    /// Return Ok(GuiRequest) if succeded to obtain the request.
    /// # Errors
    /// Return a `BridgeRecvError` based on the problem that occurred.
    /// # Panics
    /// If it cannot initialize Tokio runtime.
    pub fn take_request(&mut self) -> Result<GuiRequest, BridgeRecvError> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failure to initialize tokio runtime");

        rt.block_on(recv_over(&mut self.stream))
    }

    /// Use `post_response` to respond to the frontend with an `GuiResponse::Err404`
    /// # Return
    /// Return Ok(()) if succeded to obtain the request.
    /// # Errors
    /// Return a `BridgeSendError` based on the problem that occurred.
    /// # Panics
    /// If it cannot initialize Tokio runtime.
    pub fn post_err_not_found(self) -> Result<(), BridgeSendError> {
        self.post_response(GuiResponse::Err404)
    }

    /// Send to the frontend the passed response.
    /// It is a blocking call.
    /// # Return
    /// Return Ok(()) if succeded to obtain the request.
    /// # Errors
    /// Return a `BridgeSendError` based on the problem that occurred.
    /// # Panics
    /// If it cannot initialize Tokio runtime.
    pub fn post_response(mut self, response: GuiResponse) -> Result<(), BridgeSendError> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failure to initialize tokio runtime");

        rt.block_on(send_over(&mut self.stream, response))
    }
}
