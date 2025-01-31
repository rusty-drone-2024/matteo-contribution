use iced::widget::markdown;

pub struct Model {
    pub addr: String,
    pub list: Vec<String>,
    pub selected: usize,
    pub markdown: Vec<markdown::Item>,
}

impl Model {
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
    Selected(usize),
    Refresh,
}
