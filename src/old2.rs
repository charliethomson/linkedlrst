use std::fmt;


#[derive(Clone, Copy)]
struct Node<T> {
    data: Option<T>,
    next: Option<*mut Node<T>>,
} impl<T> Node<T> {
    fn new(data: T) -> Node<T> {
        Node { data: Some(data), next: None }
    }

    fn null() -> Node<T> {
        Node { data: None, next: None }
    }

    fn set_next(&mut self, next: Option<*mut Node<T>>) {
        self.next = next;
    }

    fn get_next(&mut self) -> *mut Node<T> {
        match self.next {
            None => panic!(),
            Some(n) => n
        }
    }

    fn maybe_get_next(&mut self) -> Option<*mut Node<T>> {
        self.next
    }

    fn has_next(&self) -> bool {
        match self.next {
            Some(_) => true,
            None => false,
        }
    }

    fn as_mut_ptr(&mut self) -> *mut Node<T> {
        self as *mut Node<T>
    }
} impl<T: std::marker::Copy> Node<T> {
    fn maybe_get_data(&self) -> Option<T> {
        self.data
    }

    fn get_data(&self) -> T {
        match self.data {
            Some(n) => n,
            None => panic!(),
        }
    }
} impl<T: fmt::Display> fmt::Display for Node<T> {
    
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        let mut output = (String::from("None"),String::from("None"));
        match &self.data {
            Some(n) => {output.0 = format!("Some: {}", n)},
            None    => {output.0 = String::from("None")},
        };
        match &self.next {
            Some(n) => {output.1 = String::from("Some")},
            None    => {output.1 = String::from("None")},
        };

        write!(f, "Node at address: \n\tData: {}\n\tNext node? :{}", output.0, output.1)
    }
}

struct ListIter<'a, T: 'a> {
    list: &'a LinkedList<T>,
    index: usize,
} impl<'a, T> ListIter<'a, T> {
    fn new(list: &'a LinkedList<T>) -> ListIter<'a, T> {
        ListIter {
            list: list,
            index: 0,
        }
    }
} impl<'a, T: std::marker::Copy> Iterator for ListIter<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            // let data = self.list.get(self.index);
            self.index += 1;
            None
            // data
        }
    }
}

struct LinkedList<T> {
    head: *mut Node<T>,
} impl<T> LinkedList<T> {
    unsafe fn new() -> LinkedList<T> {
        LinkedList {
            head: Node::null().as_mut_ptr(),
        }
    }

} impl<T: fmt::Display> LinkedList<T> {
    unsafe fn push(&mut self, val1: T, val2: T) {
        let mut current = self.head;
        println!("let mut current = self.head;");
        (*self.head).set_next(Some(Node::new(val1).as_mut_ptr()));
        println!("(*self.head).set_next(Some(Node::new(val1).as_mut_ptr()));");
        (*(*self.head).get_next()).set_next(Some(Node::new(val2).as_mut_ptr()));
        println!("h: {}, hn: {}, hnn: {}", *self.head, *(*self.head).get_next(), *(*(*self.head).get_next()).get_next());
    }
}   // impl <T: std::marker::Copy> LinkedList<T> {
    // unsafe fn get(&self, index: usize) -> Option<T> {
    //     eprintln!("{}", index);
    //     let mut current = self.head;
    //     for _ in 0..index {
    //         if !(*current).has_next() {
    //             return None
    //         }
    //         match (*current).maybe_get_next() {
    //             None => break,
    //             Some(n) => current = n,
    //         };
    //     }
    //     return match &(*current).data {
    //         Some(n) => Some(n),
    //         None => None
    //     };
    // }

// }
 impl<'a, T: std::marker::Copy> IntoIterator for &'a LinkedList<T> {
    type Item = T;
    type IntoIter = ListIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        ListIter::new(self)
    }
}

fn main() {
    unsafe {
        let mut n: Node<i32> = Node::new(10);

        let mut m: Node<i32> = Node::new(23);

        n.set_next(Some(m.as_mut_ptr()));

        eprintln!("{}", (*n.get_next()).get_data());

        eprintln!("n.has_next() -> {};\nm.has_next() -> {};", n.has_next(), m.has_next());

        let mut l: LinkedList<i32> = LinkedList::new();

        for n in 0..10 {
            l.push(n, n * n);
        }

        for _ in l.into_iter() {}

    }
}

// pub struct Node<T> {
//     data: Option<T>,
//     next: Option<Box<Node<T>>>
// } impl<T> Node<T> {
//     fn new(data: T) -> Node<T> {
//         Node { data: Some(data), next: None }
//     }

//     fn null() -> Node<T> {
//         Node { data: None, next: None }
//     }
// }


// struct ListIter<T> {
//     current: &Node<T>
// } impl<T> ListIter<T> {
//     fn new(list: &mut LinkedList<T>) -> ListIter<T> {
//         ListIter { current: &list.head }
//     }
    
//     fn walk(&mut self) -> bool {
//         match (*self.current).next {
//             Some(n) => {
//                 self.current = &mut *n;
//                 true
//             },
//             None    => {
//                 false
//             }
//         }
//     }
// }


// pub struct LinkedList<T> {
//     head: Node<T>
// } impl<T> LinkedList<T> {
//     fn new() -> LinkedList<T> {
//         LinkedList { head: Node::null() }
//     }

//     fn append(value: T) {

//     }
// }
// // impl Iterator for LinkedList<T> {

// // }
