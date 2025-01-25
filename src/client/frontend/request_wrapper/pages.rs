use crate::client::frontend::request_wrapper::RequestWrapper;
use common_structs::message::Link;
use wg_2024::network::NodeId;

impl RequestWrapper {
    pub(super) fn create_homepage(list: &Vec<(NodeId, Vec<Link>)>) -> String {
        let mut inner_html = String::new();

        for (server_id, server_files) in list {
            inner_html.push_str(&format!("<h2>{server_id}</h2>"));
            for link in server_files {
                inner_html.push_str(&format!("<a href=\"/file/{link}\">{link}</a><br>"));
            }
        }

        format!(
            "<!DOCTYPE html>\
            <html><body>\
                <h1>Link possibles</h1>\
                {inner_html}\
            </body></html>"
        )
    }
}
