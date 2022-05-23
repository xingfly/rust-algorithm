// 类型别名，简化长度
type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    size: usize,
    head: Link<T>,
}

struct Node<T> {
    element: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            size: 0,
            head: None,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        0 == self.size
    }

    pub fn push(&mut self, element: T) {
        // 创建一个新Node
        let node = Box::new(Node {
            element,
            // 设置下一个节点
            // take取出当前节点，并赋值给next
            next: self.head.take(),
        });
        // 当前头部设置为新Node
        self.head = Some(node);
        // 长度+1
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        // 调用take方法，移出当前Top节点，留下一个None
        // 调用map将Box<Node<T>>转换为T，并且修改size属性和head指向的位置
        // 返回Option<T>
        self.head.take().map(|node| {
            // 链表头指向当前节点的下一个节点
            self.head = node.next;
            // 大小-1
            self.size -= 1;
            // 返回元素
            node.element
        })
    }

    pub fn peek(&self) -> Option<&T> {
        // as_ref()返回不可变引用，map将&Box<Node<T>>转换为&T
        // 最终包装为Option<&T>进行返回
        self.head.as_ref().map(|node| &node.element)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        // as_mut()返回可变引用，map将&mut Box<Node<T>>转换为&mut T
        // 最终包装为Option<&mut T>进行返回
        self.head.as_mut().map(|node| &mut node.element)
    }

    // 获取所有权的迭代器方法实现
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    // 不可变引用迭代器方法实现
    pub fn iter(&self) -> Iter<T> {
        Iter {
            // Next为Top Node（类型是不可变引用&T）
            next: self.head.as_deref(),
        }
    }

    // 可变引用迭代器方法实现
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            // Next为Top Node（类型是可变引用&mut T）
            next: self.head.as_deref_mut(),
        }
    }
}

// 定义IntoIter结构体，IntoIter会转移所有权
pub struct IntoIter<T>(List<T>);
impl<T> Iterator for IntoIter<T> {
    // 关联类型设置为T
    type Item = T;
    // 迭代时调用next，通过pop获取值
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

// 定义Iter结构体，Iter为不可变引用迭代器，所以要标注生命周期
// 'a表示结构体所引用的Node 至少跟结构体活的一样长
pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    // 关联类型是 &T（标注了生命周期）
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // next返回值为Option<Self::Item>
        // map方法是Option的方法，参数是一个方法，支持从A到B的转换
        // 在这个方法中A是&Node<T> 转换后的B是&T
        // 最终返回值是Option<&T>，和关联类型一致
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.element
        })
    }
}
// 定义IterMut结构体，IterMut是可变引用迭代器，所以要标注生命周期
// 'a表示结构体所引用的Node至少和结构体存活时间一样长
pub struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    // 关联类型是 可变引用 &mut T（标注了生命周期）
    type Item = &'a mut T;
    // 调用Option的take方法，take方法的引用必须为mut类型。
    // 借用情况下可以调用take方法，指向Option的可变引用可以调用take。
    // 通过map方法将 &mut Node<T> 转换为 &mut T
    // 最后返回Option<&mut T>，和关联类型一致
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.element
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();
        // 如果link还有值就一直调用take，到link是None为止
        while let Some(mut node) = link {
            link = node.next.take();
        }
    }
}

pub fn list_test_1() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.peek(), Some(&2));
    assert_eq!(list.peek_mut(), Some(&mut 2));
    println!("链表的长度：{}", list.size())
}
