use super::TextServer;
use common_structs::message::{FileWithData, Link};
use std::collections::HashMap;

impl TextServer {
    pub(super) fn init_files() -> Vec<Link> {
        let res = [
            "https://www.google.com",
            "https://www.codewithfaraz.com/preview/master-html-css-and-javascript-by-building-a-digital-clock",
            "https://wiki.archlinux.org/title/Main_page",
            "localfile.with.image",
            "localfile.with.embed",
            "localfile.text",
            "https://www.youtube.com/?app=desktop",
            "https://theuselessweb.com/",
        ];
        
        res.iter().map(|&s| s.to_string()).collect()
    }

    pub(super) fn get_files_list(&self) -> Vec<Link> {
        self.files.clone()
    }

    pub(super) fn get_file(&self, link: &Link) -> Option<FileWithData> {
        /* TODO reenable in the future
        if !self.get_files_list().contains(link) {
            return None;
        }
        */

        if link.starts_with("localfile") {
            return Some(self.test_local_file(link));
        }

        let resp = attohttpc::get(link).send().ok()?;

        if !resp.is_success() {
            return None;
        }
        
        //TODO related data
        Some(FileWithData { file: resp.text().ok()?, related_data: HashMap::default() })
    }
    
    fn test_local_file(&self, link: &Link) -> FileWithData{
        let mut content = "";
        let mut related_data = HashMap::new();
        if link == "localfile.with.image" {
            content = "<img src=\"http://localhost:7710/file/test.jpg\"\
                style=\"width: 100%;\"></img>";
            related_data.insert("test.jpg".to_string(), 30);
        }
        if link == "localfile.with.embed" {
            content = "<iframe width=\"100%\" height=\"375\"\
                src=\"https://www.youtube.com/embed/_vhf0RZg0fg\"\
                frameborder=\"0\" allow=\"autoplay; encrypted-media\"\
                allowfullscreen></iframe>";
            related_data.insert("test.jpg".to_string(), 30);
        }

        let file = format!(
            "<!DOCTYPE html><html><body><h1>{} - {}</h1><p>\
            sdadsadsadsasadasdsdasdsadsadsadasdasdsadsadasdsdasdsa<br>\
            asdsadsadddddddddddddddddddddddddddsdsadsadsadsadasdsad<br>\
            asdsdasdsaaaadsdasdsadsadsadsadasdsadsadsadsadsadsadsad<br>\
            <p>{}</body></html>",
            self.node_id, link, content
        );

        FileWithData { file, related_data }
    }
}
