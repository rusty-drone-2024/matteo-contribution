use crate::model::Message::Selected;
use crate::model::{Message, Model};
use iced::widget::{button, row, text, Column};
use iced::{Element, Fill};

pub fn view(model: &Model) -> Element<Message> {
    let mut res = vec![];

    for (i, name) in model.list.iter().enumerate() {
        let txt = if i == model.selected {
            format!("{name} is selected")
        } else {
            name.to_string()
        };

        res.push(button(text(txt)).on_press(Selected(i)).width(Fill).into());
    }

    row![Column::from_vec(res).width(200), text(&model.log),]
        .height(Fill)
        .into()
}
