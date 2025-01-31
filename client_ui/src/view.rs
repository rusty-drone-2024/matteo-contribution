use crate::model::Message::Selected;
use crate::model::{Message, Model};
use iced::widget::{button, container, row, scrollable, text, Column};
use iced::{Element, Fill, Renderer, Theme};

type El<'a> = Element<'a, Message, Theme, Renderer>;

pub fn view(model: &Model) -> Element<Message> {
    let mut res: Vec<El> = vec![];

    for (i, name) in model.list.iter().enumerate() {
        let txt = if i == model.selected {
            format!("{name} is selected")
        } else {
            name.to_string()
        };

        res.push(button(text(txt)).on_press(Selected(i)).width(Fill).into());
    }

    row![
        Column::from_vec(res).width(200).spacing(2),
        scrollable(
            container(text(&model.log).width(Fill))
                .padding(20)
                .width(Fill)
        )
        .width(Fill),
    ]
    .height(Fill)
    .into()
}
