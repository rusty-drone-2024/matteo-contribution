use super::MediaServer;
use common_structs::message::Media;

impl MediaServer {
    pub(super) fn get_test_media() -> Option<Media> {
        // TODO actual media checks
        let url = "http://www.photo-paysage.com/albums/userpics/10001/Cascade_-15.JPG";
        let resp = attohttpc::get(url).send().ok()?;

        if !resp.is_success() {
            return None;
        }
        resp.bytes().ok()
    }
}
