use crate::model::{Message, Model};
use client_bridge::{GuiRequest, GuiResponse};
use iced::Task;
use std::future::Future;
use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::pin::Pin;
use std::task::{Context, Poll};

pub fn update(model: &mut Model, message: Message) -> Task<Message> {
    match message {
        Message::Selected(idx) => {
            model.selected = idx;
            let res = send(&model.addr, GuiRequest::Get(model.list[idx].clone()));
            if let Ok(stream) = res {
                return Task::future(Receiver::new(stream));
            }
        },
        Message::TextEditorAction(ac) => {
            model.content.perform(ac);
        },
        Message::Backend(response) => {
            println!("{response:?}");
        }
    }

    Task::none()
}

fn send(addr: &String, request: GuiRequest) -> io::Result<TcpStream>{
    let mut stream = TcpStream::connect(addr)?;
    let serialized = serde_json::to_vec(&request)?;
    stream.write_all(&serialized)?;
    Ok(stream)
}

pub struct Receiver{
    stream: TcpStream,
    temp: String,
}

impl Receiver {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream, temp: String::default() }
    }
}

impl Future for Receiver{
    type Output = Message;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut tmp = self.temp.clone();
        let Ok(_) = self.stream.read_to_string(&mut tmp) else {
            return Poll::Pending;
        };

        let Ok(response) = serde_json::from_str::<GuiResponse>(&self.temp) else {
            self.temp = tmp;
            return Poll::Pending;
        };
        
        Poll::Ready(Message::Backend(response))
    }
}