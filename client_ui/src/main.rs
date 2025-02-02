#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
use crate::model::{ClientUI, Message};
use iced::theme::palette::{Background, Extended, Pair, Primary, Secondary};
use iced::theme::Palette;
use iced::{Color, Task, Theme};
use std::env;

mod model;
mod update;
mod view;

/// # Panics
/// if no arg is passed
pub fn main() {
    let addr = env::args().nth(1).expect("NO ARGUMENT").to_string();

    let title = "Matteo Text & Media Client";
    let theme = custom_theme();
    let initial_task = Task::done(Message::Refresh);
    let initialization = || (ClientUI::new(addr, vec![]), initial_task);

    let _ = iced::application(title, ClientUI::update, ClientUI::view)
        .theme(move |_| theme.clone())
        .run_with(initialization);
}

fn custom_theme() -> Theme {
    let new_light = Palette {
        primary: Color::parse("#77c6ef").unwrap(),
        ..Palette::LIGHT
    };

    let generator = |new_light: Palette| {
        let base = new_light.background;
        let text = new_light.text;
        let primary = new_light.primary;

        let lgrey = Color::parse("#eff0f1").unwrap();
        let dgrey = Color::parse("#dee0e2").unwrap();
        let primary_strg = Color::parse("#5085a0").unwrap();
        let secondary = Color::parse("#bbe3f7").unwrap();
        let secondary_strg = Color::parse("#a8c2d0").unwrap();

        let pair = |color| Pair { color, text };

        Extended {
            background: Background {
                base: pair(base),
                weak: pair(lgrey),
                strong: pair(dgrey),
            },
            primary: Primary {
                base: pair(primary),
                weak: pair(primary),
                strong: pair(primary_strg),
            },
            secondary: Secondary {
                base: pair(secondary),
                weak: pair(secondary),
                strong: pair(secondary_strg),
            },
            is_dark: false,
            ..Extended::generate(new_light)
        }
    };

    Theme::custom_with_fn("Custom".to_string(), new_light, generator)
}
