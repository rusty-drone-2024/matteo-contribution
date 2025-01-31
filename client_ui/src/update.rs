use crate::model::{Message, Model};
use client_bridge::send::{recv_over, send_over};
use client_bridge::{GuiRequest, GuiResponse};
use iced::widget::markdown;
use std::net::TcpStream;
use iced::Task;

pub fn update(model: &mut Model, message: Message) -> Task<Message>{
    match message {
        Message::Selected(idx) => {
            model.selected = idx;

            let req = GuiRequest::Get(model.list[idx].to_string());
            match communicate(model, req) {
                GuiResponse::Err404 => {
                    model.markdown = markdown::parse("# ERROR 404").collect();
                }
                GuiResponse::GotFile(file) => {
                    model.markdown = markdown::parse(&file.file).collect();
                }
                _ => {}
            }
        }
        Message::LinkClicked(url) => {
            println!("LINK CLICKED {url:?}");
            let searched = &url.to_string();
            if let Some(pos) = model.list.iter().position(|el| el == searched) {
                return Task::done(Message::Selected(pos));
            }
        }
        Message::Refresh => {
            let req = GuiRequest::ListAll;
            match communicate(model, req) {
                GuiResponse::ListOfAll(list) => {
                    let mut final_list = vec![];
                    for (_, el) in list {
                        final_list.extend(el.into_iter());
                    }
                    model.list = final_list;
                }
                _ => {}
            }
        }
    }
    Task::none()
}

fn communicate(model: &mut Model, request: GuiRequest) -> GuiResponse {
    // TODO not single threaded
    let mut stream = TcpStream::connect(&model.addr).unwrap();
    send_over(&mut stream, request);
    recv_over::<GuiResponse>(&mut stream).unwrap()
}

/*
impl Future for Receiver {
    type Output = Message;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("TRYING");
        let mut tmp = [0; 16];
        self.stream.set_nodelay(true).unwrap();
        self.stream.set_read_timeout(Some(Duration::from_millis(100))).unwrap();
        println!("TRYING2");
        let mut res;
        loop{
            res =  self.stream.read(&mut tmp);
            if res.is_ok() {
                break;
            }
            println!("PENDING 1");
            //return Poll::Pending;
        }println!("TRYING3");

        let res = res.unwrap();
        println!("tmp {tmp:?} {res}");
        self.partial.extend(tmp.into_iter().take(res));

        let Ok(response) = serde_json::from_slice::<GuiResponse>(&self.partial) else {
            println!("PENDING 2");
        return Poll::Pending;
        };

        Poll::Ready(Message::Backend(response))
    }
}
*/
