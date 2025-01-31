use crate::client::GuiResponse;
use client_bridge::GuiResponse;
use iced::widget::text_editor::{Action, Content};

pub struct Model{
    pub addr: String,
    pub list: Vec<String>,
    pub content: Content,
    pub selected: usize,
}

impl Model {
    pub fn new(addr: String) -> Self {
        Self{
            addr,
            list: vec!["Ciao".to_string(), "Ciao2".to_string()],
            content: Default::default(),
            selected: 0,
        }
    }
}



#[derive(Debug, Clone)]
pub enum Message {
    TextEditorAction(Action),
    Selected(usize),
    Backend(GuiResponse),
}