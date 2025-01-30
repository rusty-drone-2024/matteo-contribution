use rusty_drones::RustyDrone;
use rusty_tester::utils::Network;
use std::thread::sleep;
use std::time::Duration;
use rusty_drones_servers::TextServer as SamText;
use rusty_drones_servers::MediaServer as SamMedia;
use rusty_drones_servers::ChatServer as SamChat;
use matteo_contribution::{MediaServer, TextMediaClient, TextServer};

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

    net.create_and_run_leaf::<TextServer>(11).unwrap();
    net.create_and_run_leaf::<MediaServer>(12).unwrap();

    net.create_and_run_leaf::<SamText>(13).unwrap();
    net.create_and_run_leaf::<SamMedia>(14).unwrap();

    net.create_and_run_leaf::<SamChat>(15).unwrap();

    loop {
        sleep(Duration::from_secs(1));
    }
}
