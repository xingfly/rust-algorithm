#[derive(Debug)]
struct Stack<T> {
    top: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            top: 0,
            data: Vec::new(),
        }
    }

    fn push(&mut self, value: T) {
        self.data.push(value);
        self.top += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            return None;
        }
        self.top -= 1;
        self.data.pop()
    }

    fn peek(&self) -> Option<&T> {
        if self.top == 0 {
            return None;
        }
        self.data.get(self.top - 1)
    }

    fn is_empty(&self) -> bool {
        self.top == 0
    }

    fn size(&self) -> usize {
        self.top
    }
}

// 栈测试
pub fn stack_test_1() {
    let mut stack = Stack::new();
    for i in 0..3 {
        stack.push(i);
    }
    println!("top {:?}, size{}", stack.peek().unwrap(), stack.size());
    println!("pop {:?}, size{}", stack.pop().unwrap(), stack.size());
    println!("is_empty:{}, stack:{:?}", stack.is_empty(), stack);
}

// 括号匹配
pub fn stack_test_2() {
    // let a = "()(())";
    // let b = "()((()";
    // let d = "(){}[]";
    // let e = "(){){]";
    let f = "(1+2)=(2+1)";
    let g = "(1+2}={2+1)";
    let r1 = par_checker(f);
    let r2 = par_checker(g);
    println!("{r1}");
    println!("{r2}");
}

fn par_match(open: char, close: char) -> bool {
    // opens和closers中的每个符号对应符号位置不能出错
    let opens = "([{";
    let closers = ")]}";
    opens.find(open) == closers.find(close)
}

fn par_checker(par_list: &str) -> bool {
    let mut list = Vec::new();
    for item in par_list.chars() {
        list.push(item);
    }
    let mut stack = Stack::new();
    let mut index = 0;
    let mut balance = true;
    while index < list.len() && balance {
        let c = list[index];
        // 开括号入栈
        if c == '(' || c == '[' || c == '{' {
            stack.push(c);
        }
        // 闭括号判断是否平衡
        if ')' == c || ']' == c || '}' == c {
            // 如果第一符号就是 ) 那么可以判断为不平衡
            if stack.is_empty() {
                balance = false;
            } else {
                // 当前括号和栈顶括号是否匹配
                let top = stack.pop().unwrap();
                if !par_match(top, c) {
                    balance = false;
                }
            }
        }
        // 其他符号跳过
        index += 1;
    }
    balance && stack.is_empty()
}

// 进制转换
pub fn stack_test_3() {
    let bin_str = base_converter(233, 2);
    let hex_str = base_converter(43, 16);
    println!("10 is b{bin_str}");
    println!("43 is x{hex_str}");
}

fn base_converter(mut dec_num: u32, base: u32) -> String {
    let digits = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];
    let mut rem_stack = Stack::new();
    // 余数入栈
    while dec_num > 0 {
        let rem = dec_num % base;
        rem_stack.push(rem);
        dec_num /= base;
    }
    let mut base_str = "".to_string();
    // 栈中元素出栈组成字符串
    while !rem_stack.is_empty() {
        let rem = rem_stack.pop().unwrap() as usize;
        base_str += &digits[rem].to_string();
    }
    base_str
}

