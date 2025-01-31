use super::TextServer;
use common_structs::message::Message::{ErrNotFound, RespFile};
use common_structs::message::{FileWithData, Link};
use common_structs::types::Session;
use network::PacketMessage;
use std::collections::HashMap;
use std::thread;
use wg_2024::network::NodeId;

impl TextServer {
    pub(crate) fn init_files() -> Vec<Link> {
        let res = [
            "https://www.google.com",
            "https://www.codewithfaraz.com/preview/master-html-css-and-javascript-by-building-a-digital-clock",
            "https://wiki.archlinux.org/title/Main_page",
            "localfile.with.image",
            "localfile.with.embed",
            "localfile.text",
            "https://www.youtube.com/?app=desktop",
            "https://theuselessweb.com/",
            "https://unsplash.com/",
            "https://start.duckduckgo.com/",
            "https://www.yahoo.com/?",
        ];

        res.iter().map(|&s| s.to_string()).collect()
    }

    pub(crate) fn get_files_list(&self) -> Vec<Link> {
        self.files.clone()
    }

    pub(crate) fn async_get_file(
        &self,
        link: &Link,
        session: Session,
        other_end: NodeId,
    ) -> Option<FileWithData> {
        if link.starts_with("localfile") {
            return Some(self.test_local_file(link));
        }

        //TODO LIMITS THREADS
        let sender = self.network.send.clone();
        let link = link.to_string();
        thread::spawn(move || {
            if !link.starts_with("http") {
                let _ = sender.send(PacketMessage::new(session, other_end, ErrNotFound));
                return;
            }

            let req = attohttpc::get(link);
            let Ok(resp) = req.send() else {
                let _ = sender.send(PacketMessage::new(session, other_end, ErrNotFound));
                return;
            };

            if !resp.is_success() {
                let _ = sender.send(PacketMessage::new(session, other_end, ErrNotFound));
                return;
            }

            let Ok(file) = resp.text() else {
                let _ = sender.send(PacketMessage::new(session, other_end, ErrNotFound));
                return;
            };

            //TODO related data
            let resp = FileWithData {
                file,
                related_data: HashMap::default(),
            };

            let _ = sender.send(PacketMessage::new(session, other_end, RespFile(resp)));
        });

        None
    }

    fn test_local_file(&self, link: &Link) -> FileWithData {
        let mut content = "";
        let mut related_data = HashMap::new();
        if link == "localfile.with.image" {
            content = "<img src=\"test.jpg\"\
                style=\"width: 100%;\"></img>";
            related_data.insert("test.jpg".to_string(), 12);
        }
        if link == "localfile.with.embed" {
            content = "<iframe width=\"100%\" height=\"375\"\
                src=\"https://www.youtube.com/embed/_vhf0RZg0fg\"\
                frameborder=\"0\" allow=\"autoplay; encrypted-media\"\
                allowfullscreen></iframe>";
            related_data.insert("test.jpg".to_string(), 12);
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
