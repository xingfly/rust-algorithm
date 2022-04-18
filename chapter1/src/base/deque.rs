#[derive(Debug)]
struct Deque<T> {
    cap: usize,
    data: Vec<T>,
}

impl<T> Deque<T> {
    fn new(cap: usize) -> Self {
        Deque {
            cap,
            data: Vec::with_capacity(cap),
        }
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn is_empty(&self) -> bool {
        self.cap == 0
    }

    // 从队尾添加
    fn add_rear(&mut self, val: T) -> Result<(), String> {
        if Self::size(&self) == self.cap {
            return Err("没有可用空间".to_string());
        }
        self.data.insert(0, val);
        Ok(())
    }

    // 从队尾移除
    fn remove_rear(&mut self) -> Option<T> {
        if Self::size(&self) > 0 {
            Some(self.data.remove(0))
        } else {
            None
        }
    }

    // 从队首添加
    fn add_front(&mut self, val: T) -> Result<(), String> {
        if Self::size(&self) == self.cap {
            return Err("没有可用空间".to_string());
        }
        self.data.push(val);
        Ok(())
    }

    // 从队首移除
    fn remove_front(&mut self) -> Option<T> {
        if Self::size(self) > 0 {
            self.data.pop()
        } else {
            None
        }
    }
}

pub fn deque_test_1() {
    let mut deque = Deque::new(4);
    let _r1 = deque.add_front(1);
    let _r2 = deque.add_front(2);
    let _r3 = deque.add_rear(3);
    let _r4 = deque.add_rear(4);

    if let Err(error) = deque.add_front(5) {
        println!("info:{error}");
    }

    if let Some(data) = deque.remove_rear() {
        println!("{data}")
    } else {
        println!("empty")
    }

    if let Some(data) = deque.remove_front() {
        println!("{data}")
    } else {
        println!("empty")
    }

    println!("size:{},is_empty:{}", deque.size(), deque.is_empty());
    println!("content:{:?}", deque);
}

// 回文检测
pub fn deque_test_2() {
    let pal = "abba";
    let is_pal = pal_checker(pal);
    println!("{pal}是否为回文：{is_pal}")
}

fn pal_checker(pal: &str) -> bool {
    let mut deque = Deque::new(pal.len());
    for c in pal.chars() {
        let _r = deque.add_rear(c);
    }
    let mut is_pal = true;
    while deque.size() > 1 && is_pal {
        let head = deque.remove_front();
        let tail = deque.remove_rear();
        if head != tail {
            is_pal = false;
        }
    }
    is_pal
}
