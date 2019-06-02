use std::fmt;
use std::ptr;

pub struct Node<T> {
    data: Option<T>,
    next: Option<ptr::NonNull<Node<T>>>,
}
impl<T> Node<T> {
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
        self.next = ptr::NonNull::<Node<T>>::new(next);
    }

    fn get_next(&mut self) -> ptr::NonNull<Node<T>> {
        self.next.unwrap()
    }

    fn maybe_set_next(&mut self, next: Option<*mut Node<T>>) {
        self.next = match next {
            Some(n) => ptr::NonNull::<Node<T>>::new(n),
            None => None,
        };
    }

    fn maybe_get_next(&mut self) -> Option<ptr::NonNull<Node<T>>> {
        self.next
    }

    fn as_mut_ptr(&mut self) -> *mut Node<T> {
        self as *mut Node<T>
    }

    fn as_ptr(&self) -> *const Node<T> {
        self as *const Node<T>
    }

    fn has_next(&self) -> bool {
        match self.next {
            Some(_) => true,
            None    => false,
        }
    }
}
impl<T: std::marker::Copy + fmt::Display> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Node at {:?}: \n\tdata: {};\n\tnext: {:?};\n",
            self.as_ptr(),
            match self.data {
                Some(n) => format!("{}", n),
                None => String::from("None"),
            },
            match self.next {
                Some(n) => format!("{:?}", n.as_ptr()),
                None => String::from("None"),
            }
        )
    }
}
impl<T: std::marker::Copy + fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.data {
                Some(n) => format!("{}", n),
                None => String::from("None"),
            }
        )
    }
}

struct LinkedList<T>
    where T: std::marker::Copy + fmt::Display
{
    head: Node<T>
} impl<T: std::marker::Copy + fmt::Display> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList {
            head: Node::<T>::null(),
        }
    }

    unsafe fn push(&mut self, value: T) {
        let mut current = self.head.as_mut_ptr();
        while (*current).has_next() {
            current = (*current).get_next().as_mut().as_mut_ptr();
        }
        println!("cur{:?};",(*current));
        (*current).set_next(Node::new(value).as_mut_ptr());
        println!("set next to {:?}", (*current).get_next().as_ptr());
        println!("next{:?};", (*current).get_next().as_mut());
    }
}

fn main() {
    unsafe {
        let mut n = Node::new(10);
        let mut m = Node::new(11);
        let mut o = Node::new(12);
        let mut p = Node::new(13);

        n.set_next(m.as_mut_ptr());
        m.set_next(o.as_mut_ptr());
        o.set_next(p.as_mut_ptr());

        let mut current = m.as_mut_ptr();
        println!("n{:?}\nm{:?}\no{:?}\np{:?}\n",
        n.has_next(),m.has_next(),o.has_next(),p.has_next());
        while let Some(c) = (*current).get_next() {
            println!("{:?}", (*current));
            current = (*current).get_next().as_mut().as_mut_ptr();
        }


        println!(
            "Display: m: {}, n: {};\nDebug: m: {:?}, n: {:?}",
            m, n, m, n
        );

        // let mut l = LinkedList::<i32>::new();

        // println!("l.push(10)");
        // l.push(10);
        // println!("l.push(11)");
        // l.push(11);
        // println!("l.push(12)");
        // l.push(12);
        // println!("l.push(13)");
        // l.push(13);
    }
}
