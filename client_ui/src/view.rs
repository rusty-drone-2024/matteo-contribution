use crate::model::Message::Selected;
use crate::model::{ClientUI, Message};
use iced::advanced::widget::text::Text;
use iced::border::Radius;
use iced::widget::button::{Status, Style};
use iced::widget::{button, column, container, markdown, row, scrollable, text, Column};
use iced::{
    Background, Border, Center, Color, Element, Fill, FillPortion, Renderer, Shadow, Theme,
};

type El<'a> = Element<'a, Message, Theme, Renderer>;

impl ClientUI {
    fn btn_style(theme: &Theme, status: Status, selected: bool) -> Style {
        let base: Style = Style {
            background: Some(Background::Color(
                theme.extended_palette().primary.base.color,
            )),
            text_color: theme.palette().text,
            border: Border {
                color: theme.palette().text,
                radius: Radius::from(12),
                width: 0.0,
            },
            shadow: Shadow::default(),
        };

        match (selected, status) {
            (true, _) => base,
            (_, Status::Pressed) => Style {
                background: Some(Background::Color(
                    theme.extended_palette().primary.strong.color,
                )),
                ..base
            },
            (_, Status::Hovered) => Style {
                background: Some(Background::Color(
                    theme.extended_palette().secondary.strong.color,
                )),
                ..base
            },
            (_, Status::Active) => Style {
                background: Some(Background::Color(
                    theme.extended_palette().secondary.base.color,
                )),
                ..base
            },
            _ => Style::default(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let header: El = container(row![
            Text::new("File list")
                .size(20)
                .width(Fill)
                .height(FillPortion(1))
                .align_y(Center),
            container(
                button("Refresh")
                    .style(|t, s| Self::btn_style(t, s, false))
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
                    .style(move |t, s| Self::btn_style(t, s, selected))
                    .on_press(Selected(i))
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
