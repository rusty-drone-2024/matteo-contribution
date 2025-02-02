mod update;
mod view;

use client_bridge::GuiResponse;
use iced::task::Handle;
use iced::widget::markdown;

pub use view::style::custom_theme;

pub struct ClientUI {
    pub addr: String,
    pub list: Vec<String>,
    pub older_task: Option<Handle>,
    pub selected: usize,
    pub markdown: Vec<markdown::Item>,
}

impl ClientUI {
    pub fn new(addr: String, list: Vec<String>) -> Self {
        Self {
            addr,
            list,
            older_task: None,
            selected: 0,
            markdown: vec!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    LinkClicked(markdown::Url),
    NetResponse(GuiResponse),
    Selected(usize),
    Refresh,
}
