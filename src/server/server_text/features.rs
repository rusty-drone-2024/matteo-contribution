use super::TextServer;
use common_structs::message::{FileWithData, Link};
use std::collections::HashMap;

impl TextServer {
    pub(super) fn init_files() -> Vec<Link> {
        vec![
            "https:://www.filebello.com".to_string(),
            "https:://www.filebello2.com".to_string(),
            "marco".to_string(),
        ]
    }

    pub(super) fn get_files_list(&self) -> Vec<Link> {
        self.files.clone()
    }

    pub(super) fn get_file(&self, link: &Link) -> Option<FileWithData> {
        if !self.get_files_list().contains(link) {
            return None;
        }

        let mut img = "";
        let mut related_data = HashMap::new();
        if link == "marco" {
            img = "<img src=\"http://localhost:7710/?link=test.jpg\"\
                style=\"width: 100%;\"></img>";
            related_data.insert("test.jpg".to_string(), 30);
        }

        let file = format!(
            "<!DOCTYPE html><html><body><h1>{} - {}</h1><p>\
            sdadsadsadsasadasdsdasdsadsadsadasdasdsadsadasdsdasdsa<br>\
            asdsadsadddddddddddddddddddddddddddsdsadsadsadsadasdsad<br>\
            asdsdasdsaaaadsdasdsadsadsadsadasdsadsadsadsadsadsadsad<br>\
            <p>{}</body></html>",
            self.node_id, link, img
        );

        Some(FileWithData { file, related_data })
    }
}
