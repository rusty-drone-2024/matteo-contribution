use common_structs::leaf::LeafCommand;
use common_structs::leaf::LeafCommand::{AddSender, Kill, RemoveSender};
use crate::backend::network::NetworkBackend;

impl NetworkBackend{
    pub(super) fn handle_command(&mut self, command: LeafCommand) -> bool{
        match command {
            RemoveSender(connection_id) => {
                self.packets_out.remove(&connection_id);
            }
            AddSender(connection_id, sender) => {
                self.packets_out.insert(connection_id, sender);
            }
            Kill => { 
                return true;
            },
        }
        
        return false;
    }
}