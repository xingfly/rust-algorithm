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

impl<T> StackLink<T> {
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

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    fn iter(&self) -> Iter<T> {
        Iter {
            next: self.top.as_deref()
        }
    }

    fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.top.as_deref_mut()
        }
    }
}

struct IntoIter<T>(StackLink<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.data
        })
    }
}

struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.data
        })
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
    println!("is_empty:{},stack:{:?}", s.is_empty(), s);

    for item in s.iter() {
        println!("{item}");
    }

    for item in s.iter_mut() {
        *item = *item + 1;
        println!("{item}");
    }

    for item in s.into_iter() {
        println!("{item}");
    }
}