use crate::ui::{ClientUI, Message};
use iced::advanced::widget::text::Text;
use iced::widget::{button, column, container, markdown, row, scrollable, text, Column};
use iced::{
    Center, Color, Element, Fill, FillPortion, Renderer, Theme,
};
use style::btn_style;

pub mod style;


type El<'a> = Element<'a, Message, Theme, Renderer>;

impl ClientUI {
    pub fn view(&self) -> Element<Message> {
        let header: El = container(row![
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
        .into();

        let mut res: Vec<El> = vec![];

        for (i, name) in self.list.iter().enumerate() {
            let selected = i == self.selected;

            res.push(
                button(text(name))
                    .padding(9)
                    .style(move |t, s| btn_style(t, s, selected))
                    .on_press(Message::Selected(i))
                    .width(Fill)
                    .into(),
            );
        }

        let md: El = markdown::view(
            &self.markdown,
            markdown::Settings::default(),
            markdown::Style::from_palette(Theme::Light.palette()),
        )
        .map(Message::LinkClicked);

        row![
            container(column![
                header,
                Column::from_vec(res).spacing(6).padding(6).width(Fill)
            ])
            .width(200)
            .height(Fill)
            .style(|t| t.extended_palette().background.weak.color.into()),
            container(scrollable(container(md).padding(20).width(Fill)).width(Fill))
                .height(Fill)
                .style(|_| container::background(Color::parse("#F8FAFC").unwrap())),
        ]
        .height(Fill)
        .into()
    }
}
