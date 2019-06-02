use std::fmt;

#[derive(Clone, Copy)]
pub struct Node<T>
where
    T: std::marker::Copy,
{
    data: Option<T>,
    next: Option<*mut Node<T>>,
}
impl<T: std::marker::Copy> Node<T> {
    fn new(data: T) -> Node<T> {
        Node {
            data: Some(data),
            next: None,
        }
    }

    fn null() -> Node<T> {
        Node {
            data: None,
            next: None,
        }
    }

    fn set_next(&mut self, next: *mut Node<T>) {
        self.next = Some(next);
    }

    fn maybe_set_next(&mut self, next: Option<*mut Node<T>>) {
        self.next = next;
    }

    fn get_next(&mut self) -> *mut Node<T> {
        self.next.unwrap()
    }

    fn maybe_get_next(&mut self) -> Option<*mut Node<T>> {
        self.next
    }

    fn has_next(&mut self) -> bool {
        match self.next {
            Some(_) => true,
            None => false,
        }
    }

    fn get_data(&mut self) -> T {
        self.data.unwrap()
    }

    fn maybe_get_data(&mut self) -> Option<T> {
        self.data
    }

    fn has_data(&mut self) -> bool {
        match self.data {
            Some(_) => true,
            None => false,
        }
    }

    fn set_data(&mut self, data: T) {
        self.data = Some(data);
    }

    fn maybe_set_data(&mut self, data: Option<T>) {
        self.data = data;
    }

    fn as_mut_ptr(&mut self) -> *mut Node<T> {
        self as *mut Node<T>
    }
}
impl<T: fmt::Display + std::marker::Copy> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Node at {:?}; has_next: {}; has_data: {}",
            self as *const Node<T>,
            match self.next {
                Some(_) => "true",
                None => "false",
            },
            match self.data {
                Some(_) => "true",
                None => "false",
            }
        )
    }
}

struct LinkedList<T>
where
    T: std::marker::Copy,
{
    head: *mut Node<T>,
}
impl<T: std::marker::Copy + fmt::Display> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList {
            head: Node::<T>::null().as_mut_ptr(),
        }
    }

    unsafe fn push_back(&mut self, data: T) {
        let mut new = Node::new(data);

        let mut current = self.head;

        eprintln!("{:?}/{}", current, (*current));
        while (*current).has_next() {
            current = (*current).get_next();
        }

        (*current).set_next(new.as_mut_ptr());
    }
}

fn main() {
    unsafe {
        let mut l = LinkedList::<i32>::new();

        l.push_back(10);
        l.push_back(11);
        l.push_back(12);
        l.push_back(13);
    }
}
