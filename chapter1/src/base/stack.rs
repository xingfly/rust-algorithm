use std::collections::HashMap;

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

// 前中后缀表达式
pub fn stack_test_4() {
    let infix = "( 1 + 2 ) * ( 1 + 2 )";
    let postfix = infix_to_postfix(infix);
    match postfix {
        Some(val) => {
            println!("中缀表达式:{} -> 后缀表达式:{}", infix, val);
            let res = postfix_eval(&val);
            match res {
                Some(val) => {
                    println!("结果：{}", val);
                }
                None => {
                    println!("无法计算")
                }
            }
        }
        None => {
            println!("中缀表达式有误")
        }
    }
}

fn infix_to_postfix(infix: &str) -> Option<String> {
    // 括号匹配检查
    if !par_checker(infix) {
        return None;
    }
    let mut prec = HashMap::new();
    prec.insert("(", 1);
    prec.insert(")", 1);
    prec.insert("+", 2);
    prec.insert("-", 2);
    prec.insert("*", 3);
    prec.insert("/", 3);

    let mut op_stack = Stack::new();
    let mut postfix = Vec::new();
    for token in infix.split_whitespace() {
        // 0-9 和 A-Z范围字符入栈
        if ("A" <= token && token <= "Z") || ("0" <= token && token <= "9") {
            // push到后缀集合
            postfix.push(token);
        } else if "(" == token {
            // 开符号，入栈op_stack
            op_stack.push(token);
        } else if ")" == token {
            // 闭符号
            // 从操作符栈中弹出
            // 如果不是开符号
            // 将操作符中的(+、-、*、/)push到后缀表达式集合中
            let mut top = op_stack.pop().unwrap();
            while top != "(" {
                postfix.push(top);
                top = op_stack.pop().unwrap();
            }
        } else {
            // 如果是(+、-、*、/)这些符号
            // 比较符号优先级决定操作符是否加入postfix
            // 操作符栈不为空 && 当前栈顶的操作符优先级>=当前符号的优先级

            // 栈顶操作符
            // let op_stack_top_token = op_stack.peek().unwrap();
            // 栈顶操作符优先级
            // let op_stack_top_token_prec = prec[op_stack_top_token];
            // 当前操作符优先级
            // let current_token_prec = prec[token];
            // 栈顶操作符优先级 >= 当前操作符优先级
            while (!op_stack.is_empty()) && (prec[op_stack.peek().unwrap()] >= prec[token]) {
                // 栈顶操作符号
                let op_stack_top_token = op_stack.pop().unwrap();
                // 将栈顶操作符号push到后缀表达式集合中
                postfix.push(op_stack_top_token);
            }
            // 向操作符栈push当前操作符
            op_stack.push(token);
        }
    }
    // 剩下操作符push到postfix
    while !op_stack.is_empty() {
        postfix.push(op_stack.pop().unwrap());
    }
    // 出栈组成字符串
    let mut postfix_str = "".to_string();
    for c in postfix {
        postfix_str += &c.to_string();
        postfix_str += " ";
    }
    Some(postfix_str)
}

fn postfix_eval(postfix: &str) -> Option<i32> {
    // 小于5个字符是非法表达式
    if postfix.len() < 5 {
        return None;
    }
    let mut op_stack = Stack::new();
    for token in postfix.split_whitespace() {
        if "0" <= token && token <= "9" {
            // 数字push到栈
            op_stack.push(token.parse::<i32>().unwrap());
        } else {
            // 弹出栈中的两个数字
            let op2 = op_stack.pop().unwrap();
            let op1 = op_stack.pop().unwrap();
            // 使用操作符计算结果
            let res = do_calc(token, op1, op2);
            // 将结果push回栈
            op_stack.push(res);
        }
    }
    Some(op_stack.pop().unwrap())
}

fn do_calc(op: &str, op1: i32, op2: i32) -> i32 {
    if "+" == op {
        op1 + op2
    } else if "-" == op {
        op1 - op2
    } else if "*" == op {
        op1 * op2
    } else {
        if 0 == op2 {
            panic!("ZeroDivisionError")
        }
        op1 / op2
    }
}
