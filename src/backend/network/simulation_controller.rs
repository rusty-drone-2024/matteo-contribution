use crate::backend::network::NetworkBackend;
use common_structs::leaf::LeafCommand;
use common_structs::leaf::LeafCommand::{AddSender, Kill, RemoveSender};

impl NetworkBackend {
    pub(super) fn handle_command(&mut self, command: LeafCommand) -> bool {
        match command {
            RemoveSender(connection_id) => {
                self.packets_out.remove(&connection_id);
            }
            AddSender(connection_id, sender) => {
                self.packets_out.insert(connection_id, sender);
            }
            Kill => {
                return true;
            }
        }

        false
    }
}
