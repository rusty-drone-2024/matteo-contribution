pub struct Model {
    pub addr: String,
    pub list: Vec<String>,
    pub selected: usize,
    pub log: String,
}

impl Model {
    pub fn new(addr: String, list: Vec<String>) -> Self {
        Self {
            addr,
            list,
            selected: 0,
            log: String::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Selected(usize),
}
