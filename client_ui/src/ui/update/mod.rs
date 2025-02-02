use crate::ui::{ClientUI, Message};
use client_bridge::{GuiRequest, GuiResponse};
use iced::widget::markdown;
use iced::Task;

mod communication;

impl ClientUI {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Selected(idx) => {
                if self.selected != idx {
                    self.selected = idx;

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
            Message::NetResponse(resp) => match resp {
                GuiResponse::Err404 => {
                    self.markdown = markdown::parse("# ERROR 404").collect();
                }
                GuiResponse::GotFile(file) => {
                    self.markdown = markdown::parse(&file.file).collect();
                }
                GuiResponse::ListOfAll(list) => {
                    let mut final_list = vec![];
                    for (_, el) in list {
                        final_list.extend(el.into_iter());
                    }
                    self.list = final_list;
                }
                GuiResponse::GotMedia(_) => {
                    eprintln!("INVALID RESPONSE {resp:?}");
                }
            },
        }
        Task::none()
    }
}
