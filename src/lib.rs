#![warn(clippy::pedantic)]
mod backend;
mod client;
mod server;

pub use client::TextMediaClient;
pub use server::MediaServer;
pub use server::TextServer;
