mod handle_client;

use crossbeam_channel::{select, Receiver, Sender};
use crate::low_level::{ClientNetworkRequest, ClientNetworkResponse};

pub struct NetworkHandler{
    client_receiver: Receiver<ClientNetworkRequest>,
    client_sender: Sender<ClientNetworkResponse>,
}

impl NetworkHandler{
    pub fn new(
        client_receiver: Receiver<ClientNetworkRequest>,
        client_sender: Sender<ClientNetworkResponse>,
    ) -> Self{
        Self{client_receiver, client_sender}
    }

    pub fn run(&mut self) {
        loop {
            select! {
                recv(self.client_receiver) -> res => {
                    if let Ok(client_req) = res{
                        self.handle_client_request(client_req);
                    }
                },
            }
        }   
    }
}

