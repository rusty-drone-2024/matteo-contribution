use crate::{GuiRequest, GuiResponse, RequestWrapper};
use std::io;
use std::io::{Read, Write};

impl RequestWrapper {
    pub fn take_request(&mut self) -> Option<GuiRequest> {
        let mut string = String::new();
        self.stream.read_to_string(&mut string).ok()?;
        serde_json::from_str(&string).ok()
    }

    pub fn post_err_not_found(self) -> Option<()> {
        self.post_response(GuiResponse::Err404)
    }

    pub fn post_response(self, response: GuiResponse) -> Option<()> {
        if let Err(err) = self.send(response) {
            eprintln!("Coundn't send response because {err:?}");
            return None;
        }
        Some(())
    }

    fn send(mut self, response: GuiResponse) -> io::Result<()> {
        let serialized = serde_json::to_vec(&response)?;
        self.stream.write_all(&serialized)
    }
}
