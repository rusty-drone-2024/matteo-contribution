use crate::ui::{ClientUI, Message};
use client_bridge::{GuiRequest, GuiResponse};
use iced::widget::markdown;
use iced::Task;
use std::process::exit;

mod communication;

impl ClientUI {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Selected(idx) => {
                if self.selected != Some(idx) {
                    self.selected = Some(idx);

                    let req = GuiRequest::Get(self.list[idx].to_string());
                    return self.create_task(req);
                }
            }
            Message::Refresh => {
                let req = GuiRequest::ListAll;
                return self.create_task(req);
            }
            Message::LinkClicked(url) => {
                println!("LINK CLICKED {url:?}");
                let searched = &url.to_string();
                if let Some(pos) = self.list.iter().position(|el| el == searched) {
                    return Task::done(Message::Selected(pos));
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

    fn handle_net_resp(&mut self, resp: GuiResponse) -> Task<Message> {
        match resp {
            GuiResponse::Err404 => {
                self.markdown = markdown::parse("# ERROR 404").collect();
            }
            GuiResponse::GotFile(file) => {
                self.markdown = markdown::parse(&file.file).collect();
            }
            GuiResponse::ListOfAll(list) => {
                let list = list.into_iter().flat_map(|(_, l)| l);
                self.list = list.collect();
                
                self.selected = None;
                self.markdown.clear();
            }
            GuiResponse::GotMedia(_) => {
                eprintln!("INVALID RESPONSE {resp:?}");
            }
        }
        
        Task::none()
    }
}
