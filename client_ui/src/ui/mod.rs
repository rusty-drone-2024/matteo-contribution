mod update;
mod view;

use client_bridge::GuiResponse;
use common_structs::message::Link;
use iced::task::Handle;
use iced::widget::markdown;
pub use view::style::custom_theme;
const BASE_PATH: &str = ".resources/matteo_cache";

pub struct ClientUI {
    pub addr: String,
    pub list: Vec<Link>,
    pub older_task: Option<Handle>,
    pub content_state: ContentState,
}

pub enum ContentState {
    Empty,
    Valid {
        index: usize,
        content: Vec<markdown::Item>,
    },
    Invalid,
    Loading {
        index: usize,
        content: Option<Vec<markdown::Item>>,
        to_load: usize,
    },
}

impl ClientUI {
    pub const fn new(addr: String) -> Self {
        Self {
            addr,
            list: vec![],
            older_task: None,
            content_state: ContentState::Empty,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    LinkClicked(markdown::Url),
    NetResponse(Option<GuiResponse>),
    Selected(usize),
    Refresh,
}
