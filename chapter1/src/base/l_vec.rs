use std::fmt::Debug;

#[derive(Debug)]
struct Node<T> {
    ele: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

impl<T> Node<T> {
    fn new(ele: T) -> Node<T> {
        Node {
            ele,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LVec<T> {
    size: usize,
    head: Link<T>,
}

impl<T: Copy + Debug> LVec<T> {
    fn new() -> Self {
        LVec {
            size: 0,
            head: None,
        }
    }

    fn clear(&mut self) {
        self.size = 0;
        self.head = None;
    }

    fn len(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn push(&mut self, ele: T) {
        let node = Node::new(ele);
        // 如果是空
        if self.is_empty() {
            self.head = Some(Box::new(node));
        } else {
            // 如果已有数据
            let mut current = self.head.as_mut().unwrap();
            // 由于 0..x 不包含 x，所以self.size为最后一个，self.size-1 为倒数第二个
            for _ in 0..self.size - 1 {
                // 倒数第二个的next为current，即current就是最后一个节点
                current = current.next.as_mut().unwrap();
            }
            // 为最后一个节点的next添加元素
            current.next = Some(Box::new(node));
        }
        self.size += 1;
    }

    fn append(&mut self, other: &mut Self) {
        // 遍历other
        while let Some(node) = other.head.as_mut().take() {
            // 将other中的数据放到self末尾
            self.push(node.ele);
            // 修改other的head为other的next节点
            other.head = node.next.take();
        }
        // 清空other
        other.clear();
    }

    fn insert(&mut self, mut index: usize, ele: T) {
        if index >= self.size {
            index = self.size;
        }
        let mut node = Node::new(ele);
        // 没数据时
        if self.is_empty() {
            self.head = Some(Box::new(node));
        } else if index == 0 {
            // 在第一个位置
            node.next = self.head.take();
            self.head = Some(Box::new(node));
        } else {
            // 在其他位置时
            let mut current = self.head.as_mut().unwrap();
            for _ in 0..index - 1 {
                current = current.next.as_mut().unwrap();
            }
            node.next = current.next.take();
            current.next = Some(Box::new(node));
        }
        self.size += 1;
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.size {
            return None;
        }
        let mut node;
        // 删除第一个时
        if 0 == index {
            node = self.head.take().unwrap();
            self.head = node.next.take();
        } else {
            // 删除其他位置时
            let mut current = self.head.as_mut().unwrap();
            for _ in 0..index - 1 {
                current = current.next.as_mut().unwrap();
            }
            node = current.next.take().unwrap();
            current.next = node.next.take();
        }
        self.size -= 1;
        Some(node.ele)
    }

    fn pop(&mut self) -> Option<T> {
        if self.size >= 1 {
            self.remove(self.size - 1)
        } else {
            None
        }
    }

    fn println_l_vec(&self) {
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            println!("val:{:#?}", node.ele);
            current = node.next.as_ref();
        }
    }

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref()
        }
    }

    fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_deref_mut()
        }
    }
}

struct IntoIter<T>(LVec<T>);

impl<T: Copy + Debug> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.remove(0)
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
            &node.ele
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
            &mut node.ele
        })
    }
}

pub fn l_vec_test_1() {
    let mut l_vec = LVec::new();
    l_vec.push(10);
    l_vec.push(11);
    l_vec.push(12);
    l_vec.push(13);
    l_vec.insert(0, 9);

    let mut l_vec_2 = LVec::new();
    l_vec_2.insert(0, 8);
    l_vec_2.append(&mut l_vec);
    println!("l_vec_2 len: {}", l_vec_2.len());
    l_vec_2.println_l_vec();

    let res1 = l_vec_2.pop();
    let res2 = l_vec_2.remove(0);
    println!("弹出 - 尾部 - {:#?}", res1.unwrap());
    println!("删除 - 头部 - {:#?}", res2.unwrap());
    l_vec_2.println_l_vec();

    for item in l_vec_2.iter() {
        println!("{}", *item)
    }

    for item in l_vec_2.iter_mut() {
        println!("{}", *item)
    }

    for item in l_vec_2.into_iter() {
        println!("{}", item)
    }
}
