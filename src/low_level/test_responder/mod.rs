use crossbeam_channel::{Receiver, Sender};
use wg_2024::packet::Packet;

pub struct EchoServer {
    packet_recv: Receiver<Packet>,
    packet_send: Sender<Packet>,
}

impl EchoServer {
    pub fn new(packet_recv: Receiver<Packet>, packet_send: Sender<Packet>) -> Self
    where
        Self: Sized,
    {
        Self {
            packet_recv,
            packet_send,
        }
    }

    pub fn run(&mut self) {
        while let Ok(packet) = self.packet_recv.recv() {
            self.packet_send.send(packet).unwrap()
        }
    }
}

/*
let list = vec![
    ("https:://www.filebello.com".to_string(), 5),
    ("https:://www.filebello2.com".to_string(), 3),
    ("marco".to_string(), 2),
];
 */
