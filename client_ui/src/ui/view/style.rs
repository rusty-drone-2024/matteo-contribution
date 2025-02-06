use iced::border::Radius;
use iced::theme::palette::{Background, Extended, Pair, Primary, Secondary};
use iced::theme::Palette;
use iced::widget::button::{Status, Style};
use iced::{Border, Color, Padding, Shadow, Theme};

pub fn btn_style(theme: &Theme, status: Status, selected: bool) -> Style {
    let base: Style = Style {
        background: Some(iced::Background::Color(
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
            background: Some(iced::Background::Color(
                theme.extended_palette().primary.strong.color,
            )),
            ..base
        },
        (_, Status::Hovered) => Style {
            background: Some(iced::Background::Color(
                theme.extended_palette().secondary.strong.color,
            )),
            ..base
        },
        (_, Status::Active) => Style {
            background: Some(iced::Background::Color(
                theme.extended_palette().secondary.base.color,
            )),
            ..base
        },
        _ => Style::default(),
    }
}

pub fn pad_xy(x: f32, y: f32) -> Padding {
    Padding {
        top: y,
        right: x,
        bottom: y,
        left: x,
    }
}

pub fn custom_theme() -> Theme {
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
