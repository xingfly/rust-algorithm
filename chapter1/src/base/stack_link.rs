#[derive(Debug, Clone)]
struct Node<T> {
    data: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data,
            next: None,
        }
    }
}

#[derive(Debug, Clone)]
struct StackLink<T> {
    size: usize,
    top: Link<T>,
}

impl<T: Clone> StackLink<T> {
    fn new() -> Self {
        StackLink {
            size: 0,
            top: None,
        }
    }

    fn push(&mut self, ele: T) {
        let mut node = Node::new(ele);
        node.next = self.top.take();
        self.top = Some(Box::new(node));
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        self.top.take().map(|node| {
            self.size -= 1;
            let node = *node;
            self.top = node.next;
            node.data
        })
    }

    fn peek(&self) -> Option<&T> {
        self.top.as_ref().map(|node| {
            &node.data
        })
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.top.as_mut().map(|node| {
            &mut node.data
        })
    }

    fn size(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }
}

pub fn stack_link_test_1() {
    let mut s = StackLink::new();
    s.push(1);
    s.push(2);
    s.push(3);
    if let Some(v) = s.peek_mut() {
        *v = 4
    }
    println!("top {:?} size {}", s.peek().unwrap(), s.size());
    println!("top {:?} size {}", s.pop().unwrap(), s.size());
    println!("is_empty:{},stack:{:?}", s.is_empty(), s)
}