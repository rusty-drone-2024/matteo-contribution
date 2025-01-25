#![warn(clippy::pedantic)]
mod backend;
mod client;
mod server;

use crate::server::{MediaServer, TextServer};
use client::TextMediaClient;
use rusty_drones::RustyDrone;
use rusty_tester::utils::Network;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let topology = &[
        (0, 1),
        (11, 6),
        (12, 10),
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

    let mut net = Network::create_and_run::<RustyDrone>(13, topology, &[0, 11, 12]);

    net.create_and_run_leaf::<TextMediaClient>(0).unwrap();
    net.create_and_run_leaf::<TextServer>(11).unwrap();
    net.create_and_run_leaf::<MediaServer>(12).unwrap();

    loop {
        sleep(Duration::from_secs(1));
    }
}
