use crate::ui::{ClientUI, Message};
use client_bridge::{GuiRequest, GuiResponse};
use iced::Task;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::mem;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

impl ClientUI {
    pub(super) fn create_task(&mut self, req: GuiRequest) -> Task<Message> {
        let addr = self.addr.clone();
        let task = Task::perform(communicate(addr, req), Message::NetResponse);
        let (task, handle) = task.abortable();

        let old = mem::replace(&mut self.older_task, Some(handle));
        if let Some(old) = old {
            old.abort();
        }

        task
    }
}

async fn communicate(addr: String, request: GuiRequest) -> GuiResponse {
    let mut stream = TcpStream::connect(addr).await.unwrap();
    send_over(&mut stream, request).await.expect("LOL1");
    recv_over::<GuiResponse>(&mut stream).await.expect("LOL2")
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
