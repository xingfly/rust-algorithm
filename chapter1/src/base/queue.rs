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
// 队列基础使用
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

// 烫手山芋问题
pub fn queue_test_2() {
    let names = vec!["1", "2", "3", "4", "5", "6"];
    let rem = hot_potato(names, 8);
    println!("最后剩下：{rem}");
}

fn hot_potato(names: Vec<&str>, num: usize) -> &str {
    let mut queue = Queue::new(names.len());
    for name in names {
        let _ = queue.enqueue(name);
    }
    while queue.size() > 1 {
        // 在num没有达到前，每个人从队首出队，再重新入队到队尾
        for _i in 0..num {
            let name = queue.dequeue().unwrap();
            let _rm = queue.enqueue(name);
        }
        // 到达num后，队首的人出队
        let _rm = queue.dequeue();
    }
    queue.dequeue().unwrap()
}
