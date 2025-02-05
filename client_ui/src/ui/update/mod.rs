use crate::ui::{ClientUI, ContentState, Message};
use client_bridge::{GuiRequest, GuiResponse};
use common_structs::message::{FileWithData, Link, Media};
use iced::widget::markdown;
use iced::Task;
use std::fs::write;
use std::path::Path;
use std::process::exit;

mod communication;

impl ClientUI {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Selected(idx) => {
                let link = self.select(idx);

                if let Some(link) = link {
                    let req = GuiRequest::Get(link);
                    return self.create_task(req);
                }
            }
            Message::Refresh => {
                let req = GuiRequest::ListAll;
                return self.create_task(req);
            }
            Message::LinkClicked(url) => {
                let url = url.to_string();

                if url.starts_with("http") {
                    let _ = open::that(url.as_str());
                } else {
                    // TODO maybe invert order check
                    let pos = self.list.iter().position(|x| url.eq(x));
                    if let Some(pos) = pos {
                        return Task::done(Message::Selected(pos));
                    }
                }
            }
            Message::NetResponse(resp) => {
                let Some(resp) = resp else {
                    exit(0);
                };
                return self.handle_net_resp(resp);
            }
        }
        Task::none()
    }

    fn select(&mut self, idx: usize) -> Option<Link> {
        if let ContentState::Valid { index, .. } = self.content_state {
            if index == idx {
                return None;
            }
        }

        let link = self.list.get(idx)?;
        self.content_state = ContentState::Loading {
            index: idx,
            content: None,
            to_load: 1,
        };

        Some(link.to_string())
    }

    fn handle_net_resp(&mut self, resp: GuiResponse) -> Task<Message> {
        match resp {
            GuiResponse::Err404 => {
                self.content_state = ContentState::Invalid;
            }
            GuiResponse::ListOfAll(list) => {
                let list = list.into_iter().flat_map(|(_, l)| l);
                self.list = list.collect();
                self.content_state = ContentState::Empty;
            }
            GuiResponse::GotFile(file_link, file) => {
                let media = self.handle_got_file(file_link, &file);

                return self
                    .create_batch_task(media.into_iter().map(|link| GuiRequest::GetMedia(link)));
            }
            GuiResponse::GotMedia(link, media) => {
                self.handle_got_media(&link, media);
            }
        }

        Task::none()
    }

    fn handle_got_file(&mut self, _link: Link, file: &FileWithData) -> Vec<Link> {
        let ContentState::Loading { index, .. } = self.content_state else {
            eprintln!("Received file when non waiting");
            return vec![];
        };

        let to_load = file.related_data.len();
        let content = markdown::parse(&file.file).collect();

        self.content_state = if to_load > 0 {
            let content = Some(content);
            ContentState::Loading {
                index,
                content,
                to_load,
            }
        } else {
            ContentState::Valid { index, content }
        };

        file.related_data.keys().cloned().collect()
    }

    fn handle_got_media(&mut self, link: &Link, media: Media) {
        let ContentState::Loading {
            index,
            content,
            to_load,
        } = &mut self.content_state
        else {
            return eprintln!("Received media when non waiting");
        };

        // TODO remove hardcode (+ mkdir)
        let _ = write(
            Path::new(&format!(
                "/home/matteo/.cache/matteo_contribution_img/{link}"
            )),
            media,
        );
        *to_load -= 1;
        if *to_load > 0 {
            return;
        }

        self.content_state = ContentState::Valid {
            index: *index,
            content: content.clone().unwrap_or_default(),
        }
    }
}
