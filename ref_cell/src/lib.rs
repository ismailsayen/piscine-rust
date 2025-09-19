mod messenger;
use std::{cell::RefCell, rc::Rc};

pub use messenger::Tracker;

impl Tracker {
    pub fn new(max: usize) -> Self {
        Tracker {
            messages: RefCell::new(vec![]),
            value: RefCell::new(0),
            max: max,
        }
    }
    pub fn peek(&self, v: &Rc<u32>) {
        *self.value.borrow_mut() = Rc::strong_count(v);
        let quotta = (Rc::strong_count(v) * 100) / self.max;

        self.messages.borrow_mut().push(format!(
            "Info: This value would use {:?}% of your quota",
            quotta
        ));
    }
    pub fn set_value(&self, v: &Rc<u32>) {
        if Rc::strong_count(v) > self.max {
            self.messages
                .borrow_mut()
                .push(format!("Error: You can't go over your quota!"));
        } else {
            let quotta = (Rc::strong_count(v) * 100) / self.max;
            self.messages.borrow_mut().push(format!(
                "Warning: You have used up over {:?}% of your quota!",
                quotta
            ));
        }
    }
}
