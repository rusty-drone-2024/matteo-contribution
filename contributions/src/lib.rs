#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

mod client;
mod server_media;
mod server_text;

pub use client::TextMediaClient;
pub use server_media::MediaServer;
pub use server_text::TextServer;
