#[derive(Debug)]
pub struct Stack<T> {
    top: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            top: 0,
            data: Vec::new(),
        }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
        self.top += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            return None;
        }
        self.top -= 1;
        self.data.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        if self.top == 0 {
            return None;
        }
        self.data.get(self.top - 1)
    }

    pub fn is_empty(&self) -> bool {
        self.top == 0
    }

    pub fn size(&self) -> usize {
        self.top
    }
}

pub fn stack_test_1() {
    let mut stack = Stack::new();
    for i in 0..3 {
        stack.push(i);
    }
    println!("top {:?}, size{}", stack.peek().unwrap(), stack.size());
    println!("pop {:?}, size{}", stack.pop().unwrap(), stack.size());
    println!("is_empty:{}, stack:{:?}", stack.is_empty(), stack);
}
