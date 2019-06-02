
use std::{
    alloc::{alloc, dealloc, Layout},
    ptr::{write, read},
    default::Default,
};

#[derive(Debug)]
struct Node<T> {
    ptr: *mut Self,
    next: Option<*mut Self>,
    last: Option<*mut Self>,
    value: T,
} impl<T: Default + Copy> Node<T> {
    fn new(value: T) -> Self {
        unsafe {
            let ptr = alloc(Layout::new::<Self>()) as *mut Self;
            let node = Node {
                ptr,
                next: None,
                last: None,
                value,
            };
            write(ptr, node.clone());
            node
        }
    }

    fn with_next(self, next: Option<*mut Self>) -> Self {
        let mut new = self.clone();
        new.next = next;
        return new;
    }

    fn with_last(self, last: Option<*mut Self>) -> Self {
        let mut new = self.clone();
        new.last = last;
        return new;
    }

    fn with_value(self, value: T) -> Self {
        let mut new = self.clone();
        new.value = value;
        return new;
    }

    fn from_ptr(ptr: *mut Self) -> Self {
        unsafe {
            let node = read(ptr);
            Node {
                ptr,
                next: None,
                last: None,
                value: T::default(),
            }.with_next(node.next)
             .with_last(node.last)
             .with_value(node.value)
        }
    }

} impl<T: Copy + Default> Clone for Node<T> {
    fn clone(&self) -> Self {
        Node::from_ptr(self.ptr).with_value(self.value).with_next(self.next).with_last(self.last)
    }
}

fn main() {
    let mut two = Node::new(15);
    let mut one = Node::new(10).with_next(Some(two.ptr));
    let from_ptr = Node::from_ptr(two.ptr);

    eprintln!("{:?}", from_ptr);
}