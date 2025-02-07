#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::cargo_common_metadata)]
//! A backend network handler used by my three contributions.
use common_structs::message::Message;
use common_structs::types::Session;
use wg_2024::network::NodeId;

mod assembler;
mod backend;
mod disassembler;
mod topology;

pub use backend::NetworkBackend;
// TODO remove it?
pub use backend::NetworkCommunication;
pub use backend::NetworkOutput;

#[derive(Debug)]
pub struct PacketMessage {
    pub session: Session,
    pub opposite_end: NodeId,
    pub message: Message,
}

impl PacketMessage {
    #[must_use]
    pub const fn new(session: Session, opposite_end: NodeId, message: Message) -> Self {
        Self {
            session,
            opposite_end,
            message,
        }
    }
}
