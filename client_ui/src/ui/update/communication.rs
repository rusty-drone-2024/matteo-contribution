use crate::ui::Message::NetResponse;
use crate::ui::{ClientUI, Message};
use client_bridge::send::{recv_over, send_over};
use client_bridge::{GuiRequest, GuiResponse};
use iced::Task;
use std::mem;
use tokio::net::TcpStream;

impl ClientUI {
    pub(super) fn create_task(&mut self, req: GuiRequest) -> Task<Message> {
        let addr = self.addr.clone();
        let task = Task::perform(communicate(addr, req), NetResponse);
        self.make_abortable(task)
    }

    pub(super) fn create_batch_task(
        &mut self,
        requests: impl IntoIterator<Item = GuiRequest>,
    ) -> Task<Message> {
        let tasks = requests.into_iter().map(|req| {
            let addr = self.addr.clone();
            Task::perform(communicate(addr, req), NetResponse)
        });

        self.make_abortable(Task::batch(tasks))
    }

    fn make_abortable(&mut self, task: Task<Message>) -> Task<Message> {
        let (task, handle) = task.abortable();

        let old = mem::replace(&mut self.older_task, Some(handle));
        if let Some(old) = old {
            old.abort();
        }

        task
    }
}

async fn communicate(addr: String, request: GuiRequest) -> Option<GuiResponse> {
    let mut stream = TcpStream::connect(addr).await.unwrap();
    send_over(&mut stream, request).await.ok()?;
    recv_over::<GuiResponse>(&mut stream).await.ok()
}
