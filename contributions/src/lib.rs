#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::cargo_common_metadata)]
//! Crate that contains my three personal contributions.
//! A text server, a media server and most importantly a client
//! for text and media in format of markdown. 

mod client;
mod server_media;
mod server_text;

pub use client::TextMediaClient;
pub use server_media::MediaServer;
pub use server_text::TextServer;
