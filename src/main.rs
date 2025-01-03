mod local_server;
mod low_level;
mod media_client;

use crossbeam_channel::unbounded;
use media_client::MediaClient;
use wg_2024::network::NodeId;

fn main() {
    let (controller_send, _) = unbounded();
    let (_, controller_rcv) = unbounded();
    let (_packet_leaf_in, packet_rcv) = unbounded();
    let (packet_send, _packet_leaf_out) = unbounded();

    let mut client = MediaClient::new(
        7,
        controller_send,
        controller_rcv,
        packet_rcv,
        vec![(11 as NodeId, packet_send)].into_iter().collect(),
    );

    client.run();
}

/*
fn run_dummy_network(req_channel: Receiver<FrontendClientRequest>, resp_chanel: Sender<FrontendClientResponse>){
    while let Ok(request) = req_channel.recv(){
        match request {
            FrontendClientRequest::ListAll => {
                let list = vec![("https:://www.filebello.com".to_string(), 5), ("https:://www.filebello2.com".to_string(), 3)];
                let _ = resp_chanel.send(FrontendClientResponse::ListOfAll(list.into_iter().collect()));
            }
            FrontendClientRequest::Get(_, _) => {}
        }
    }
}

*/

/*
impl FrontendServer{
    pub(super) fn handle_list_of_all(&self, list: HashMap<Link, NodeId>) -> String{
        let mut html = "<!DOCTYPE html><html><body><h1>Link possibles</h1>".to_string();
        for link in list.keys(){
            html.push_str(&format!("<a href=\"localhost:7838/{}\">{}</a><br>", link, link));
        }

        html.push_str("</body></html>");
        html.to_string()
    }
}*/
