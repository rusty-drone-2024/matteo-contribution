use client_bridge::GuiResponse;
use iced::widget::markdown;

pub struct ClientUI {
    pub addr: String,
    pub list: Vec<String>,
    pub selected: usize,
    pub markdown: Vec<markdown::Item>,
}

impl ClientUI {
    pub fn new(addr: String, list: Vec<String>) -> Self {
        Self {
            addr,
            list,
            selected: 0,
            markdown: markdown::parse("").collect(),
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
