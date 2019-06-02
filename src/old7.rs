mod linked_list {
    use std::{
        alloc::{alloc_zeroed, dealloc},
        fmt::{Debug, Display, Formatter, Result as fmt_Result},
        iter::{FromIterator, Iterator},
        ptr::{read, write, NonNull},
        io::{ErrorKind, Error},
    };

    struct Node<T> {
        value: T,
        next: Option<NonNull<Node<T>>>,
    }
    impl<T> Node<T> {
        fn new(value: T) -> Self {
            Node {
                value,
                next: None,
            }
        }

        fn with_next(mut self, next: NonNull<Self>) -> Self {
            self.set_next(next);
            self
        }

        fn as_non_null(&mut self) -> Result<NonNull<Self>, Error> {
            match NonNull::new(self.as_mut_ptr()) {
                Some(n) => Ok(n),
                None    => Err(Error::new(ErrorKind::Other, String::from("Attempted to create NonNull from null pointer")))
            }
        }
        
        fn as_mut_ptr(&mut self) -> *mut Self {
            self as *mut Self
        }

        fn has_next(&self) -> bool {
            match self.next {
                Some(_) => true,
                None    => false,
            }
        }

        fn get_next(&self) -> Result<NonNull<Self>, Error> {
            match self.next {
                Some(n) => Ok(n),
                None    => Err(Error::new(ErrorKind::Other, String::from("Unable to get next node where node does not exist")))
            }
        }

        fn set_next(&mut self, next: NonNull<Self>) {
            self.next = Some(next);
        }

        fn get_value(&self) -> T where T: Clone {
            self.value.clone()
        }

        fn set_value(&mut self, value: T) -> T where T: Clone {
            let tmp = self.value.clone();
            self.value = value;
            return tmp;
        }
    }

    pub struct ListIter<T> {
        list: LinkedList<T>,
        current: Option<NonNull<Node<T>>>,
    } impl<T> ListIter<T> {
        fn new(list: LinkedList<T>) -> ListIter<T> {
            let head = list.head;
            ListIter {
                list,
                current: head,
            }
        }

        fn return_ownership(self) -> LinkedList<T> {
            self.list
        }
    } impl<T> Iterator for ListIter<T> where T: Clone {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            match self.current {
                Some(node_ptr) => unsafe {
                    let out = node_ptr.as_ref().get_value();
                    self.current = match node_ptr.as_ref().get_next() {
                        Ok(next) => Some(next),
                        _ => None,
                    };
                    return Some(out);
                },
                _ => None,
            }
        }
    }
    
    pub struct LinkedList<T> {
        head: Option<NonNull<Node<T>>>,
        tail: Option<NonNull<Node<T>>>,
        len: usize,        
    } impl<T> LinkedList<T> {
        pub fn new() -> Self {
            LinkedList {
                head: None,
                tail: None,
                len: 0,
            }
        }

        pub fn from_vec(v: &Vec<T>) -> Self where T: Clone {
            let mut list = Self::new();
            for value in v.iter() { list.push(value.clone()); }
            return list;
        }

        pub fn to_vec(self) -> Vec<T> where T: Clone {
            let mut v = Vec::<T>::with_capacity(self.len);
            for (index, item) in self.into_iter().enumerate() { v.insert(index, item); }
            return v;
        }

        pub fn cloned_into_vec(&self) -> Vec<T> where T: Clone {
            return Self::to_vec(self.clone());
        }

        pub fn len(&self) -> usize {
            self.len
        }

        pub fn push(&mut self, value: T) {
            match self.head {
                Some(n) => { 
                    let mut current = Some(n);
                    unsafe {
                        let mut next_ptr = current.unwrap().as_ref().get_next();
                        while next_ptr.is_ok() && current.unwrap() != next_ptr.unwrap() {
                            current = Some(next_ptr.unwrap());
                            next_ptr = current.unwrap().as_ref().get_next();
                        }
                    }
                    let mut last = current.unwrap();
                    eprintln!("{:p}", last);
                    unsafe { eprintln!("{}", last.as_ref().has_next()); }
                    let last = unsafe { last.as_mut() };
                    let new_node = match Node::new(value).as_non_null() {
                        Ok(n) => n,
                        Err(e) => panic!(format!("Error pushing value to linked list: {}", e))
                    };
                    last.set_next(new_node);
                },
                None => {
                    let new_node = match Node::new(value).as_non_null() {
                        Ok(n) => n,
                        Err(e) => panic!(format!("Error pushing value to linked list: {}", e))
                    };
                    eprintln!("{:p}", new_node);
                    self.head = Some(new_node);
                }
            }
            self.len += 1;
        }

        pub fn pop() {}
    } impl<T: Clone> IntoIterator for LinkedList<T> {
        type IntoIter = ListIter<T>;
        type Item = T;

        fn into_iter(self) -> Self::IntoIter {
            ListIter::new(self)
        }
    } impl<T: Clone> Clone for LinkedList<T> {
        fn clone(&self) -> Self {
            let mut list: LinkedList<T> = LinkedList::new();
            let mut current = self.head;
            unsafe {
                while let Ok(next_ptr) = current.unwrap().as_ref().get_next() {
                    list.push(next_ptr.as_ref().get_value());
                    current = Some(next_ptr);
                }
            }
            return list;
        }
    }
}

fn main() {
    use linked_list::LinkedList;
    let mut l = LinkedList::new();
    l.push(10);
    l.push(10);
    l.push(10);
    l.push(10);
    eprintln!("{}", l.len());
}