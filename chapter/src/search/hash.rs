fn hash1(str: &str, size: usize) -> usize {
    let mut sum = 0;
    for c in str.chars() {
        sum += c as usize;
    }
    sum % size
}

fn hash2(str: &str, size: usize) -> usize {
    let mut sum = 0;
    for (i, c) in str.chars().enumerate() {
        // 修改不同位置的权重，减少冲突
        sum += (i + 1) * (c as usize);
    }
    sum % size
}

// 哈希函数测试
pub fn hash_test_1() {
    let size = 11;
    let s1 = "rust";
    let s2 = "Rust";
    let p1 = hash1(s1, size);
    let p2 = hash1(s2, size);
    println!("{} in slot {}, {} in slot {}", s1, p1, s2, p2);

    let size = 11;
    let s1 = "rust";
    let s2 = "Rust";
    let p1 = hash2(s1, size);
    let p2 = hash2(s2, size);
    println!("{} in slot {}, {} in slot {}", s1, p1, s2, p2);
}

// HashMap测试
pub fn hash_test_2() {
    println!("HashMap 线性探测开放寻址法 测试");
    let mut map = MyHashMap::new(11);
    map.insert(10, "cat");
    map.insert(2, "dog");
    map.insert(3, "tiger");

    println!("size:{:?}", map.len());
    println!("map contains key 2:{}", map.contains(2));
    println!("key 3:{:?}", map.get(3));
    println!("remove key 3:{:?}", map.remove(3));
    println!("remove key 3:{:?}", map.remove(3));
}


// 线性探测开放寻址法
#[derive(Debug, Clone, PartialEq)]
struct MyHashMap<T> {
    size: usize,
    // 保存键
    slot: Vec<usize>,
    // 保存值
    data: Vec<T>,
}

impl<T: Clone + PartialEq + Default> MyHashMap<T> {
    fn new(size: usize) -> Self {
        let mut slot = Vec::with_capacity(size);
        let mut data = Vec::with_capacity(size);
        for _i in 0..size {
            slot.push(0);
            data.push(Default::default());
        }
        MyHashMap {
            size,
            slot,
            data,
        }
    }

    fn hash(&self, key: usize) -> usize {
        key % self.size
    }

    fn rehash(&self, pos: usize) -> usize {
        (pos + 1) & self.size
    }

    fn insert(&mut self, key: usize, value: T) {
        if 0 == key {
            panic!("Error: key must > 0");
        }
        let pos = self.hash(key);
        // slot中无数据，可以直接插入
        if 0 == self.slot[pos] {
            self.slot[pos] = key;
            self.data[pos] = value;
        } else { // 已有数据，寻找下一个可行的位置
            let mut next = self.rehash(pos);
            while 0 != self.slot[next] && key != self.slot[next] {
                next = self.rehash(next);
                if next == pos { // 槽满了，退出
                    panic!("Error: slot is full");
                }
            }
            // 在找到的插槽插入数据
            if 0 == self.slot[next] {
                self.slot[next] = key;
                self.data[next] = value;
            } else {
                self.data[next] = value;
            }
        }
    }

    fn remove(&mut self, key: usize) -> Option<T> {
        if 0 == key {
            panic!("Error: key must > 0");
        }
        let pos = self.hash(key);
        if 0 == self.slot[pos] { // 槽中无数据，直接返回None
            None
        } else if key == self.slot[pos] { // 有数据，key和当前位置正好相等
            self.slot[pos] = 0;
            let data = Some(self.data[pos].clone());
            self.data[pos] = Default::default();
            data
        } else { // 有数据，但key和当前位置不相等
            let mut data: Option<T> = None;
            let mut stop = false;
            let mut found = false;
            let mut current = pos;
            while 0 != self.slot[current] && !found && !stop {
                if key == self.slot[current] {
                    found = true;
                    self.slot[current] = 0;
                    data = Some(self.data[current].clone());
                    self.data[current] = Default::default();
                } else {
                    current = self.rehash(current);
                    if current == pos {
                        stop = true
                    }
                }
            }
            data
        }
    }

    fn get(&self, key: usize) -> Option<&T> {
        if 0 == key {
            panic!("Error: key must > 0");
        }
        let pos = self.hash(key);
        let mut data: Option<&T> = None;
        let mut stop = false;
        let mut found = false;
        let mut current = pos;
        while 0 != self.slot[current] && !found && !stop {
            if key == self.slot[current] {
                found = true;
                data = self.data.get(current);
            } else {
                current = self.rehash(current);
                if current == pos {
                    stop = true;
                }
            }
        }
        data
    }

    fn contains(&self, key: usize) -> bool {
        if 0 == key {
            panic!("Error: key must > 0");
        }
        self.slot.contains(&key)
    }

    fn len(&self) -> usize {
        let mut len = 0;
        for d in self.slot.iter() {
            // 槽不为0表示有值
            if &0 != d {
                len += 1;
            }
        }
        len
    }
}