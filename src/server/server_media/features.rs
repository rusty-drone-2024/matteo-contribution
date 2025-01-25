use super::MediaServer;
use common_structs::message::Media;

impl MediaServer {
    pub(super) fn get_test_media() -> Option<Media> {
        // TODO actual media checks
        let url = "https://brameshtechanalysis.com/wp-content/uploads/2021/03/test.jpg";
        let resp = attohttpc::get(url).send().ok()?;

        if !resp.is_success() {
            return None;
        }
        resp.bytes().ok()
    }
}
