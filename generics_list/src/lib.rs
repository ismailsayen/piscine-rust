#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
        let new_list = match self.head.take() {
            Some(node) => Some(Node {
                value: value,
                next: Some(Box::new(node)),
            }),
            None => Some(Node {
                value: value,
                next: None,
            }),
        };
        self.head = new_list;
    }

    pub fn pop(&mut self) {
        if let Some(node) = self.head.take() {
            self.head = node.next.map(|ele| *ele)
        }
    }

    pub fn len(&self) -> usize {
        let mut coun: usize = 0;
        let mut current: Option<&Node<T>> = self.head.as_ref();
        while let Some(node) = current {
            coun += 1;
            current = node.next.as_deref();
        }

        coun
    }
}
