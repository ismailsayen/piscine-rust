use std::cell::RefCell;
#[derive(Debug)]
pub struct Tracker {
    pub messages: RefCell<Vec<String>>,
    pub value: RefCell<usize>,
    pub max: usize,
}
