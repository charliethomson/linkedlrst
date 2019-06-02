
#[macro_use] extern crate typename;
use std::{
    ptr::{null as null_ptr, read, write},
    alloc::{alloc, dealloc, Layout},
    fmt::{Debug, Display, Formatter, Result as fmt_Result},
};
use typename::TypeName;

#[derive(Copy, TypeName)]
struct Node<T> {
    next: *mut Node<T>,
    value: T,
} impl<T: Copy> Node<T> {
    fn new(value: T) -> Self {
        let node = Node {next: null_ptr::<Self>() as *mut Self, value};
        unsafe {
            let ptr = alloc(Layout::new::<Self>()) as *mut Self;
            write(ptr, node);
        }
        return node;
    }

    fn default() -> Self where T: Default {
        Node::new(T::default())
    }

    fn with_next(self, next: *mut Node<T>) -> Self {
        Node {
            value: self.value,
            next: next,
        }
    }

    fn has_next(&self) -> bool {
        !self.next.is_null()
    }

    fn set_next(&mut self, next: *mut Node<T>) {
        self.next = next;
    }

    fn get_next(&self) -> *mut Node<T> {
        if !self.has_next() {
            panic!("Cannot get node that doesn't exist");
        } else {
            self.next
        }
    }

    fn maybe_get_next(&self) -> Option<*mut Node<T>> {
        if !self.has_next() {
            None
        } else {
            Some(self.next)
        }
    }

    fn as_mut(&mut self) -> *mut Self {
        self as *mut Self
    }

    fn as_ptr(&self) -> *const Self {
        self as *const Self
    }

} impl<T: Copy> Clone for Node<T> {
    fn clone(&self) -> Self {
        Node::new(self.value).with_next(self.next)
    }
} impl<T: Copy + Default> Default for Node<T> {
    fn default() -> Self { Node::<T>::default() }
} impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt_Result {
        write!(f, "{}", self.value)
    }
} impl<T: Copy + Display + TypeName> Debug for Node<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt_Result {
        write!(f, "Node<{}>: value: {}; ", T::type_name(), self.value).unwrap();
        if self.has_next() {
            write!(f, "next: {:p};", self.next)
        } else {
            write!(f, "next: None;")
        }
    }
}


fn main() {
    unsafe {
        unsafe fn get_last(head: &Node<i32>) -> *mut Node<i32> {
            let mut cur = head.as_ptr();
            while !cur.is_null() && cur.as_ref().unwrap().maybe_get_next().is_some() {
                cur = cur.as_ref().unwrap().get_next();
            }
            cur as *mut Node<i32>
        }
        let mut head = Node::new(1);
        for i in 2..10 {
            let mut new_node = Node::new(i);
            let last = get_last(&head).as_mut();
            last.unwrap().set_next(new_node.as_mut());
        }

        let mut cur = head.as_mut();
        while !cur.is_null() && cur.as_ref().unwrap().maybe_get_next().is_some() {
            eprintln!("{:?}", cur.as_ref());
            cur = cur.as_ref().unwrap().get_next();
        }
    }
}
