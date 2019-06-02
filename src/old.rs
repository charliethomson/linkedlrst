use std::fmt;

#[derive(Debug, Clone, Copy)]

struct Node {
    data: i32,
    next: Option<*mut Node>,
} impl Node {
    fn new(data: i32) -> Node {
        Node { data: data, next: None }
    }

    fn get_next(&mut self) -> Option<Node> {
        unsafe {
            match self.next {
                Some(n) => Some(*n),
                None    => None
            }
        }
    }

    fn set_next(&mut self, next: Option<Node>) {
        self.next = match next {
            Some(mut n) => Some(&mut n),
            None        => None,
        };
    }
} impl fmt::Display for Node {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

struct LinkedList {
    head: Node,
    current: Option<Node>,
} impl LinkedList {
    fn new() -> LinkedList {
        let mut l = LinkedList { head: Node::new(0), current: None };
        l.current = Some(l.head);
        l
    }

    fn length(&mut self) -> u32 {
        let mut count: u32 = 0;
        let mut current = self.head;
        loop {
            match current.next {
                Some(_) => {
                    current = current.get_next().unwrap();
                    count += 1; },
                None    => { break; }
            };
        };

        count
    }

    fn get_last(&mut self) -> Node {
        let mut current = self.head;
        loop {
            match current.next {
                Some(_) => {
                    current = current.get_next().unwrap();
                },
                None    => {
                    break;
                }
            };
        };
        return current;

    }

    fn append(&mut self, data: i32) {
        let new = Node::new(data);
        self.get_last().set_next(Some(new));
        println!("{}, {}", data, new);
    }

    fn print(&mut self) {
        print!("[");
        loop {
            match self.current {
                Some(n) => {
                    self.current = self.current.unwrap().get_next();
                    match n.next {
                        Some(_) => { print!("{}, ", n.data); }
                        None    => { print!("{}", n.data); }
                    }
                },
                None    => {
                    self.current = Some(self.head);
                    break;
                }
            };

        };
        println!("]");

    }

} impl Iterator for LinkedList {
    
    type Item = i32;

    fn next(&mut self) -> Option<i32> {

        match self.current {
            Some(mut n) => {
                self.current = n.get_next();
                Some(n.data)
            },
            None    => {
                self.current = Some(self.head);
                None
            },
        }
    }
}

fn main() {
    let mut l = LinkedList::new();
    for i in 1..20 {
        println!("{}", i);
        l.append(i);
    }
    
    for i in l {
        println!("{}", i);
    }

}