use crate::ui::view::style::pad_xy;
use crate::ui::{ClientUI, ContentState, Message, BASE_PATH};
use iced::advanced::widget::text::Text;
use iced::widget::{button, column, container, row, scrollable, text, Column, Container};
use iced::widget::{markdown, Scrollable};
use iced::{Center, Color, Element, Fill, FillPortion, Theme};
use style::btn_style;

pub mod style;

impl ClientUI {
    pub fn view(&self) -> Element<Message> {
        let content: Container<Message> = match &self.content_state {
            ContentState::Valid { content, .. } => Self::markdown(content),
            ContentState::Empty => Self::info_message("No selection"),
            ContentState::Invalid => Self::info_message("Failed to load"),
            ContentState::Loading { .. } => Self::info_message("Loading..."),
        };

        row![self.sidebar(), content].into()
    }

    fn sidebar(&self) -> Container<Message> {
        container(column![Self::header(), self.col_of_btn()])
            .width(200)
            .height(Fill)
            .style(|t| t.extended_palette().background.weak.color.into())
    }

    fn header() -> Container<'static, Message> {
        container(row![
            Text::new("Files")
                .size(20)
                .width(Fill)
                .height(FillPortion(1))
                .align_y(Center),
            container(
                button("Refresh")
                    .style(|t, s| btn_style(t, s, false))
                    .on_press(Message::Refresh)
                    .height(35)
            )
            .height(FillPortion(1))
            .align_y(Center)
        ])
        .height(45)
        .padding(6)
        .style(|t: &Theme| t.extended_palette().background.strong.color.into())
    }

    fn col_of_btn(&self) -> Scrollable<Message> {
        #[allow(clippy::match_same_arms)]
        let selected = match self.content_state {
            ContentState::Valid { index, .. } => Some(index),
            ContentState::Loading { index, .. } => Some(index),
            _ => None,
        };

        let pad_btn = pad_xy(12.0, 9.0);
        let btns = self
            .list
            .iter()
            .enumerate()
            .map(|(i, name)| {
                let selected = Some(i) == selected;

                button(text(name))
                    .width(Fill)
                    .padding(pad_btn)
                    .style(move |t, s| btn_style(t, s, selected))
                    .on_press(Message::Selected(i))
                    .into()
            })
            .collect();

        let pad = pad_xy(12.0, 6.0);
        let list = Column::from_vec(btns).spacing(6).padding(pad);
        scrollable(list.width(Fill)).width(Fill)
    }

    fn markdown(content: &Vec<markdown::Item>) -> Container<Message> {
        let md = markdown::view(
            content,
            markdown::Settings::with_image_path(BASE_PATH.to_string()),
            markdown::Style::from_palette(Theme::Light.palette()),
        )
        .map(Message::LinkClicked);

        container(scrollable(container(md).padding(20).width(Fill)).width(Fill))
            .height(Fill)
            .style(|_| container::background(Color::parse("#F8FAFC").unwrap()))
    }

    fn info_message(message_str: &str) -> Container<Message> {
        container(
            iced::widget::text(message_str)
                .size(24)
                .height(Fill)
                .width(Fill)
                .align_y(Center)
                .align_x(Center),
        )
    }
}
