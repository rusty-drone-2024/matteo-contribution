use crate::model::Message::Selected;
use crate::model::{Message, Model};
use iced::widget::{button, container, markdown, row, scrollable, text, Column};
use iced::{Element, Fill, Renderer, Theme};

type El<'a> = Element<'a, Message, Theme, Renderer>;

pub fn view(model: &Model) -> Element<Message> {
    let header: El = container(row![container(text("File list")).width(Fill), button("R").on_press(Message::Refresh)]).into();

    let mut res: Vec<El> = vec![header];

    for (i, name) in model.list.iter().enumerate() {
        let txt = if i == model.selected {
            format!("{name} is selected")
        } else {
            name.to_string()
        };

        res.push(button(text(txt)).on_press(Selected(i)).width(Fill).into());
    }

    let md: El = markdown::view(
        &model.markdown,
        markdown::Settings::default(),
        markdown::Style::from_palette(Theme::Light.palette()),
    ).map(Message::LinkClicked);

    row![
        Column::from_vec(res).width(200).spacing(2),
        scrollable(
            container(
                md
            )
            .padding(20)
            .width(Fill)
        )
        .width(Fill),
    ]
    .height(Fill)
    .into()
}
