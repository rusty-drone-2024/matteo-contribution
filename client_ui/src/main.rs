#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
use crate::model::{ClientUI, Message};
use iced::theme::palette::{Background, Danger, Extended, Pair, Primary, Secondary, Success};
use iced::{Color, Task, Theme};
use std::env;

mod model;
mod update;
mod view;

pub fn main() {
    let addr = env::args()
        .collect::<Vec<_>>()
        .get(1)
        .expect("NO ARGUMENT")
        .to_string();

    let theme = Theme::custom_with_fn("Custom".to_string(), Theme::Light.palette(), |_| {
        let white = Color::WHITE;
        let black = Color::BLACK;
        let lgrey = Color::parse("#eff0f1").unwrap();
        let dgrey = Color::parse("#dee0e2").unwrap();
        let primary = Color::parse("#77c6ef").unwrap();
        let primary_strg = Color::parse("#5085a0").unwrap();
        let secondary = Color::parse("#bbe3f7").unwrap();
        let secondary_strg = Color::parse("#a8c2d0").unwrap();

        Extended {
            background: Background {
                base: Pair {
                    color: white,
                    text: black,
                },
                weak: Pair {
                    color: lgrey,
                    text: black,
                },
                strong: Pair {
                    color: dgrey,
                    text: black,
                },
            },
            primary: Primary {
                base: Pair {
                    color: primary,
                    text: black,
                },
                weak: Pair {
                    color: primary,
                    text: black,
                },
                strong: Pair {
                    color: primary_strg,
                    text: black,
                },
            },
            secondary: Secondary {
                base: Pair {
                    color: secondary,
                    text: black,
                },
                weak: Pair {
                    color: secondary,
                    text: black,
                },
                strong: Pair {
                    color: secondary_strg,
                    text: black,
                },
            },
            success: Success {
                base: Pair {
                    color: white,
                    text: white,
                },
                weak: Pair {
                    color: white,
                    text: white,
                },
                strong: Pair {
                    color: white,
                    text: white,
                },
            },
            danger: Danger {
                base: Pair {
                    color: white,
                    text: white,
                },
                weak: Pair {
                    color: white,
                    text: white,
                },
                strong: Pair {
                    color: white,
                    text: white,
                },
            },
            is_dark: false,
        }
    });

    let title = "Matteo Text & Media Client";
    let initial_task = Task::done(Message::Refresh);
    let initialization = || (ClientUI::new(addr, vec![]), initial_task);

    let _ = iced::application(title, ClientUI::update, ClientUI::view)
        .theme(move |_| theme.clone())
        .run_with(initialization);
}
