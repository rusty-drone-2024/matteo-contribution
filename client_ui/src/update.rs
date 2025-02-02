use crate::model::{ClientUI, Message};
use client_bridge::{GuiRequest, GuiResponse};
use iced::widget::markdown;
use iced::Task;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

impl ClientUI {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Selected(idx) => {
                if self.selected != idx {
                    self.selected = idx;

                    let req = GuiRequest::Get(self.list[idx].to_string());
                    let addr = self.addr.clone();
                    return Task::perform(communicate(addr, req), Message::NetResponse);
                }
            }
            Message::LinkClicked(url) => {
                println!("LINK CLICKED {url:?}");
                let searched = &url.to_string();
                if let Some(pos) = self.list.iter().position(|el| el == searched) {
                    return Task::done(Message::Selected(pos));
                }
            }
            Message::Refresh => {
                let req = GuiRequest::ListAll;
                let addr = self.addr.clone();
                return Task::perform(communicate(addr, req), Message::NetResponse);
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

async fn communicate(addr: String, request: GuiRequest) -> GuiResponse {
    let mut stream = TcpStream::connect(addr).await.unwrap();
    send_over(&mut stream, request).await;
    recv_over::<GuiResponse>(&mut stream).await.unwrap()
}

async fn send_over<T: Serialize>(stream: &mut TcpStream, data: T) -> Option<()> {
    let serialized = serde_json::to_vec(&data).ok()?;
    let len = serialized.len();

    stream.write_all(&len.to_le_bytes()).await.ok()?;
    stream.write_all(&serialized).await.ok()
}

async fn recv_over<T: DeserializeOwned>(stream: &mut TcpStream) -> Option<T> {
    let mut len = [0u8; size_of::<usize>()];
    stream.read_exact(&mut len).await.ok()?;
    let len = usize::from_le_bytes(len);

    let mut vec = vec![0u8; len];
    stream.read_exact(&mut vec).await.ok()?;
    serde_json::from_slice(&vec).ok()
}
