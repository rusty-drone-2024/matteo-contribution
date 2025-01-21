use common_structs::message::Message;
use common_structs::types::SessionId;
use wg_2024::network::NodeId;

mod assembler;
mod disassembler;
pub mod network;
mod topology;

#[derive(Debug)]
pub struct PacketMessage {
    pub session: SessionId,
    pub opposite_end: NodeId,
    pub message: Message,
}

impl PacketMessage {
    pub fn new(session: SessionId, opposite_end: NodeId, message: Message) -> Self {
        Self {
            session,
            opposite_end,
            message,
        }
    }
}
