use crate::client::frontend::request_wrapper::RequestWrapper;
use common_structs::message::Link;

impl RequestWrapper {
    pub(super) fn create_homepage(list: &Vec<Link>) -> String {
        let mut inner_html = String::new();
        for link in list {
            inner_html.push_str(&format!("<a href=\"/file/{link}\">{link}</a><br>"));
        }

        format!(
            "\
            <!DOCTYPE html>\
            <html><body>\
                <h1>Link possibles</h1>\
                {inner_html}\
            </body></html>"
        )
    }
}
