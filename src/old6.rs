
use std::{
    alloc::{ alloc_zeroed, dealloc, realloc, Layout },
    ptr::{ copy, write, read },
    fmt::{ Debug, Display, Formatter, Result as fmt_Result },
};

struct Node<T> {
    ptr: *mut T,
    next: Option<*mut Node<T>>,
} impl<T> Node<T> {
    unsafe fn new() -> Self {
        let ptr = alloc_zeroed(Layout::new::<T>()) as *mut T;
        Node {
            ptr, 
            next: None,
        }
    }

    unsafe fn with_value(self, val: T) -> Self {
        write(self.ptr, val);
        Node {
            ptr: self.ptr,
            next: self.next, 
        }
    }

    fn with_next(self, node: *mut Node<T>) -> Self {
        Node {
            ptr: self.ptr,
            next: Some(node),
        }
    }

    fn as_mut_ptr(&mut self) -> *mut Self {
        &mut self.clone() as *mut Node<T>
    }

    unsafe fn get_val(&self) -> T {
        read(self.ptr)
    }

    unsafe fn set_val(&self, val: T) {
        write(self.ptr, val);
    }

    unsafe fn get_next(&self) -> Node<T> {
        if !self.has_next() {
            panic!("Connot read node that doesn't exist");
        }
        match self.next {
            Some(node_ptr) => read(node_ptr),
            None           => panic!("Cannot read node that doesn't exist")
        }
    }

    unsafe fn set_next(&mut self, next: Node<T>) {
        let ptr = alloc_zeroed(Layout::new::<Node<T>>()) as *mut Node<T>;
        write(ptr, next);
        self.next = Some(ptr);
    }

    fn has_next(&self) -> bool {
        match self.next {
            Some(node_ptr) => node_ptr.is_null(),
            None           => false,
        }
    }
} impl<T> Clone for Node<T> {
    fn clone(&self) -> Self {
        let ptr = alloc_zeroed(Layout::new::<T>()) as *mut T;
        write(ptr, self.get_val());
        Node {
            ptr,
            next: self.next,
        }
    }
}

struct ListIter<T>
{
    list: LinkedList<T>,
    current: Option<Node<T>>,
} impl<T> ListIter<T>
{
    fn new(list: LinkedList<T>) -> Self {
        ListIter {
            list,
            current: None,
        }
    }
} impl<T> Iterator for ListIter<T>
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {

    }
}

struct LinkedList<T>  
{
    ptr: *mut Node<T>,
    len: usize,
} impl<T> LinkedList<T> 
{
    unsafe fn new() -> Self {
        let ptr = Node::<T>::new().as_mut_ptr();
        LinkedList {
            ptr,
            len: 0,
        }
    }

    unsafe fn append(&mut self, value: T) {
        // create the new node
        // if there are nodes
        // walk them until you've gotten to `len`
        let mut new_node = Node::new().with_value(value);
        match self.len {
            0 => self.ptr = new_node.as_mut_ptr(),
            n => {
                let mut cur = self.ptr;
                for _ in 0..n {
                    cur = (*cur).get_next().as_mut_ptr();
                }
                (*cur).set_next(new_node);
            }
        }
    }
} impl<T> IntoIterator for LinkedList<T> 
{
    type IntoIter = ListIter<T>;
    type Item = T;
    fn into_iter(self) -> Self::IntoIter {
        ListIter::new(self.clone())
    }

} impl<T> Display for LinkedList<T> where T: Display
{
    fn fmt(&self, f: &mut Formatter) -> fmt_Result {
        write!(f, "[").unwrap();
        for item in self.clone().into_iter() {
            if item.has_next() {
                write!(f, "{}, ", item).unwrap();
            } else {
                write!(f, "{}", item).unwrap();
            }
        }

        write!(f, "]")
    }
} impl<T> Clone for LinkedList<T> {

}

fn main() {
    unsafe {
        let n = Node::<i32>::new().with_value(10i32);
        eprintln!("n.get_val() = {}", n.get_val());
        n.set_val(12i32);
        eprintln!("n.get_val() = {}", n.get_val());

        let l = LinkedList::<i32>::new();
        eprintln!("LinkedList: ptr: {:p}; len: {}", l.ptr, l.len);
    }
}