#![warn(clippy::pedantic)]
mod backend;
mod client;
mod server;

pub use client::TextMediaClient;
pub use server::MediaServer;
pub use server::TextServer;

/*
use crate::server::{MediaServer, TextServer};
use client::TextMediaClient;
use rusty_drones::RustyDrone;
use rusty_drones_servers::MediaServer as SamMedia;
use rusty_drones_servers::ChatServer as SamChat;
use rusty_drones_servers::Server as SamSer;
use rusty_drones_servers::TextServer as SamText;
use rusty_tester::utils::Network;
use std::thread::sleep;
use std::time::Duration;


fn main() {
    let topology = &[
        (0, 1),
        (11, 6),
        (12, 10),
        (13, 10),
        (14, 2),
        (1, 4),
        (2, 5),
        (3, 6),
        (4, 7),
        (5, 8),
        (6, 9),
        (7, 10),
        (8, 1),
        (9, 2),
        (10, 3),
    ];

    let clients = &[0, 11, 12, 13, 14, 15];
    let mut net = Network::create_and_run::<RustyDrone>(16, topology, clients);
    // TODO wait for other parts for PDR

    net.create_and_run_leaf::<MediaServer>(11).unwrap();
    net.create_and_run_leaf::<SamSer<SamText>>(12).unwrap();
    net.create_and_run_leaf::<SamSer<SamMedia>>(13).unwrap();
    net.create_and_run_leaf::<TextServer>(14).unwrap();
    net.create_and_run_leaf::<SamSer<SamChat>>(15).unwrap();
    net.create_and_run_leaf::<TextMediaClient>(0).unwrap();

    loop {
        sleep(Duration::from_secs(1));
    }
}
*/
