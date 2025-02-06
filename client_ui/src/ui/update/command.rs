use crate::ui::{ClientUI, BASE_PATH};
use std::{fs, io};

impl ClientUI {
    pub(super) fn delete_cache() -> io::Result<()> {
        let _ = fs::remove_dir_all(BASE_PATH);
        fs::create_dir(BASE_PATH)
    }
}
