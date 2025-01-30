use common_structs::message::Message;
use common_structs::types::Session;
use wg_2024::network::NodeId;

mod assembler;
mod disassembler;
pub mod network;
mod topology;

#[derive(Debug)]
pub struct PacketMessage {
    pub session: Session,
    pub opposite_end: NodeId,
    pub message: Message,
}

impl PacketMessage {
    pub fn new(session: Session, opposite_end: NodeId, message: Message) -> Self {
        Self {
            session,
            opposite_end,
            message,
        }
    }
}
