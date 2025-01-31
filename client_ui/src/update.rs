use crate::model::{Message, Model};
use client_bridge::send::{recv_over, send_over};
use client_bridge::{GuiRequest, GuiResponse};
use std::net::TcpStream;

pub fn update(model: &mut Model, message: Message) {
    match message {
        Message::Selected(idx) => {
            model.selected = idx;

            // TODO not single threaded
            let mut stream = TcpStream::connect(&model.addr).unwrap();
            send_over(&mut stream, GuiRequest::Get(model.list[idx].to_string()));
            let res = recv_over::<GuiResponse>(&mut stream).unwrap();

            match res {
                GuiResponse::Err404 => {
                    model.log = "ERROR 404".to_string();
                }
                GuiResponse::GotFile(file) => {
                    model.log = file.file;
                }
                _ => {}
            }
        }
    }
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
