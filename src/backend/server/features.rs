use super::TextServer;
use common_structs::message::{FileWithData, Link};
use std::collections::HashMap;

impl TextServer {
    pub(super) fn get_files_list(&self) -> Vec<Link> {
        vec![
            "https:://www.filebello.com".to_string(),
            "https:://www.filebello2.com".to_string(),
            "marco".to_string(),
        ]
    }

    pub(super) fn get_file(&self, link: Link) -> Option<FileWithData> {
        //TODO self.id instead of 20
        let file = format!(
            "<!DOCTYPE html><html><body><h1>{} - {}</h1></body></html>",
            20, link
        );

        Some(FileWithData {
            file,
            related_data: HashMap::new(),
        })
    }
}
