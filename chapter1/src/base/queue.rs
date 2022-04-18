#[derive(Debug)]
struct Queue<T> {
    cap: usize,
    data: Vec<T>,
}

impl<T> Queue<T> {
    fn new(size: usize) -> Self {
        Queue {
            cap: size,
            data: Vec::with_capacity(size),
        }
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn enqueue(&mut self, val: T) -> Result<(), String> {
        if Self::size(&self) == self.cap {
            return Err("没有可用的空间".to_string());
        }
        self.data.insert(0, val);
        Ok(())
    }

    fn dequeue(&mut self) -> Option<T> {
        if Self::size(&self) > 0 {
            self.data.pop()
        } else {
            None
        }
    }

    fn is_empty(&self) -> bool {
        0 == Self::size(&self)
    }
}

pub fn queue_test_1() {
    let mut queue = Queue::new(3);
    let _r1 = queue.enqueue(1);
    let _r2 = queue.enqueue(2);
    let _r3 = queue.enqueue(3);
    if let Err(error) = queue.enqueue(4) {
        println!("Enqueue Error:{error}");
    }
    while !queue.is_empty() {
        if let Some(data) = queue.dequeue() {
            println!("data:{data}");
            println!("size:{},empty:{}", queue.size(), queue.is_empty());
        } else {
            println!("empty")
        }
    }

    println!("content:{:?}", queue);
}
