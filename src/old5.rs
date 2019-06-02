#![feature(ptr_internals)]

use std::{
    mem::{size_of},
    alloc::{Layout, alloc_zeroed, dealloc},
    ptr::{Unique, null, null_mut, replace, read, write},
    fmt::{Display, Debug, Formatter, Result as fmt_Result},
};
struct Node<T> {
    value: Option<T>,
    next: *mut Self,
}
impl<T> Node<T> {
    fn new() -> Self {
        Node::<T> {
            value: None,
            next: null_mut(),
        }
    }

    fn with_next(mut self, node: &mut Node<T>) -> Self {
        self.next = node.as_mut_ptr();
        return self;
    }

    fn with_value(mut self, value: T) -> Self {
        self.value = Some(value);
        return self;
    }

    fn walk(&mut self) -> Option<*mut Self> {
        match self.has_next() {
            false => Some(self.next),
            true => None
        }
    }

    fn as_mut_ptr(&mut self) -> *mut Self {
        let p: *mut Self = self;
        return p;
    }
    
    fn as_ptr(&self) -> *const Self {
        let p: *const Self = self;
        return p;
    }

    fn has_next(&self) -> bool {
        self.next.is_null()
    }
}
impl<T: Debug> Debug for Node<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt_Result {
        writeln!(f, "Node at {:p}:", self.as_ptr()).unwrap();
        writeln!(f, "has_next: {} -> {:p}", self.has_next(), self.next).unwrap();
        writeln!(f, "value: {:?}\n", self.value)
    }
}


struct LinkedList<T> {
    head: *mut Node<T>,
}
impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            head: Node::new().as_mut_ptr(),
        }
    }
    
    fn len(&self) -> usize {
        let current = self.head;
        let mut count = 0;
        unsafe { while let Some(current) = (*current).walk() { count += 1 } }
        return count;
    }

    fn get_last_node(&mut self) -> *mut Node<T> {
        let current = self.head;
        unsafe { while let Some(current) = (*current).walk() { () } }
        return current;
    }

    fn push_back(&mut self, value: T) {
        let mut last = self.get_last_node();
        unsafe { (*last).next = Node::new().with_value(value).as_mut_ptr() ;
        eprintln!("{:p} -> {:p}", last, (*last).next);}
    }
}

impl<T: Display> Debug for LinkedList<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt_Result {
        writeln!(f, "LinkedList at {:p}:", self.head).unwrap();
        write!(f, "values: [").unwrap();
        let current = self.head;
        unsafe {
            // eprintln!("current: {:p}, (*current).walk(): {:p}", current, (*current).walk().unwrap());
            while let Some(current) = (*current).walk() {
                match &(*current).value {
                    Some(n) => match (*current).has_next() {
                        true => write!(f, "{}, ", n).unwrap(),
                        false => write!(f, "{}", n).unwrap(),
                    },
                    _ => (),
                }
            }
        }

        writeln!(f, "]")
    }

}


fn main() {
    unsafe {
        let mut l = LinkedList::<i32>::new();
        l.push_back(10);
        eprintln!("l: {:?}", l);
    }
}