use crate::ui::{ClientUI, Message};
use iced::advanced::widget::text::Text;
use iced::widget::{button, column, container, markdown, row, scrollable, text, Column, Container};
use iced::{Center, Color, Element, Fill, FillPortion, Theme};
use style::btn_style;

pub mod style;

impl ClientUI {
    pub fn view(&self) -> Element<Message> {
        row![self.sidebar(), self.markdown(),].height(Fill).into()
    }

    fn sidebar(&self) -> Container<Message> {
        container(column![Self::header(), self.col_of_btn(),])
            .width(200)
            .height(Fill)
            .style(|t| t.extended_palette().background.weak.color.into())
    }

    fn header() -> Container<'static, Message> {
        container(row![
            Text::new("File list")
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

    fn col_of_btn(&self) -> Column<Message> {
        let btns = self
            .list
            .iter()
            .enumerate()
            .map(|(i, name)| {
                let selected = Some(i) == self.selected;

                button(text(name))
                    .padding(9)
                    .style(move |t, s| btn_style(t, s, selected))
                    .on_press(Message::Selected(i))
                    .width(Fill)
                    .into()
            })
            .collect();

        Column::from_vec(btns).spacing(6).padding(6).width(Fill)
    }

    fn markdown(&self) -> Container<Message> {
        let md = markdown::view(
            &self.markdown,
            markdown::Settings::default(),
            markdown::Style::from_palette(Theme::Light.palette()),
        )
        .map(Message::LinkClicked);

        container(scrollable(container(md).padding(20).width(Fill)).width(Fill))
            .height(Fill)
            .style(|_| container::background(Color::parse("#F8FAFC").unwrap()))
    }
}
