use std::rc::Rc;
pub struct Node {
    pub ref_list: Vec<Rc<String>>,
}

impl Node {
    pub fn new(ref_list: Vec<Rc<String>>) -> Self {
        Self { ref_list }
    }

    pub fn add_element(&mut self, element: Rc<String>) {
        self.ref_list.push(element);
    }

    pub fn rm_all_ref(&mut self, element: Rc<String>) {
        println!("{:?}",self.ref_list);
        self.ref_list = self
            .ref_list
            .iter()
            .filter(|ele| !Rc::ptr_eq(&ele, &element)).cloned().collect();
    }
}

pub fn how_many_references(ref_list: &Rc<String>) -> usize {
    Rc::strong_count(ref_list)
}
