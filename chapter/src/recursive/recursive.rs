use crate::base::stack::Stack;

// 递归

// 大问题分解小问题：将问题分解为小问题直到分解为最基本问题，然后合并这些问题的结果得到最终结果


fn nums_sum(nums: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for item in nums {
        sum += item;
    }
    sum
}

fn nums_sum_recursive_1(nums: &[i32]) -> i32 {
    if nums.len() == 1 {
        // 当数组中只有一个数据时，nums[0]实际上是初始数据的最后一个值。
        nums[0]
    } else {
        // 从小往大
        let left = nums[0];
        left + nums_sum_recursive_1(&nums[1..])
    }
}

fn nums_sum_recursive_2(nums: &[i32]) -> i32 {
    if nums.len() == 1 {
        // 代表最后一个未+的元素
        nums[0]
    } else {
        let right = nums[nums.len() - 1];
        nums_sum_recursive_2(&nums[..nums.len() - 1]) + right
    }
}

fn n_sum_1(start: i32, end: i32) -> i32 {
    if start == end {
        end
    } else {
        start + n_sum_1(start + 1, end)
    }
}

fn n_sum_2(start: i32, end: i32) -> i32 {
    if end == start {
        start
    } else {
        end + n_sum_2(start, end - 1)
    }
}


pub fn recursive_test_1() {
    let nums = vec![1, 2, 3];
    // 非递归实现
    let r1 = nums_sum(&nums);
    println!("非递归：{}", r1);
    // 递归实现-从左向右
    let r2 = nums_sum_recursive_1(&nums);
    println!("从左向右：{}", r2);
    // 递归实现-从右向左
    let r3 = nums_sum_recursive_2(&nums);
    println!("从右向左：{}", r3);
    // 递归计算从start到end的值 - 从小到大
    let r4 = n_sum_1(1, 3);
    println!("从小到大：{}", r4);
    // 递归计算从start到end的值 - 从大到小
    let r5 = n_sum_2(1, 3);
    println!("从大到小：{}", r5);
}

const BASE_STR: [&str; 16] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F"];

pub fn n_transform_base_recursive(n: i32, base: i32) -> String {
    if n < base {
        BASE_STR[n as usize].to_string()
    } else {
        let rem = (n % base) as usize;
        n_transform_base_recursive(n / base, base) + BASE_STR[rem]
    }
}

pub fn n_transform_base_stack(mut n: i32, base: i32) -> String {
    let mut stack = Stack::new();
    while n > 0 {
        let rem = n % base;
        stack.push(BASE_STR[rem as usize]);
        n /= base;
    }
    let mut result = String::from("");
    while let Some(v) = stack.pop() {
        result += v
    }
    result
}


pub fn recursive_test_2() {
    let num = 100;
    let r = n_transform_base_recursive(num, 2);
    println!("二进制:{}", r);
    let r = n_transform_base_recursive(num, 8);
    println!("八进制:{}", r);
    let r = n_transform_base_recursive(num, 16);
    println!("十六进制:{}", r);
}

// 尾递归 - 参数不保存在栈上
fn nums_sum_tail_recursion(sum: i32, nums: &[i32]) -> i32 {
    if 1 == nums.len() {
        sum + nums[0]
    } else {
        nums_sum_tail_recursion(sum + nums[0], &nums[1..])
    }
}

fn n_to_end_tail(sum: i32, start: i32, end: i32) -> i32 {
    if start == end {
        sum + start
    } else {
        n_to_end_tail(sum + start, start + 1, end)
    }
}

pub fn recursive_test_3() {
    let vec = vec![1, 2, 3, 4, 5, 6];
    let sum = nums_sum_tail_recursion(0, &vec);
    let result = n_to_end_tail(0, 1, 65535);
    println!("尾递归计算数组的和：{}", result);
    let result = n_sum_1(1, 65535);
    println!("尾递归计算数组的和：{}", result);
    println!("尾递归计算数组的和：{}", sum);
}