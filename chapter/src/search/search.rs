use std::collections::{HashMap};

// 线性查找
fn sequential_find_for_impl(arr: &[u32], target: u32) -> Option<u32> {
    for index in 0..arr.len() {
        if arr[index] == target {
            return Some(index as u32);
        }
    }
    None
}

fn sequential_find_while_impl(arr: &[u32], target: u32) -> Option<u32> {
    let mut pos = 0;
    while pos < arr.len() {
        if target == arr[pos] {
            return Some(pos as u32);
        }
        pos += 1;
    }
    None
}

// 二分查找
fn binary_search_while_impl(arr: &[u32], target: u32) -> Option<u32> {
    let mut start = 0;
    let mut end = arr.len() - 1;
    while start <= end {
        let mid = (start + end) / 2;
        let mid_value = arr[mid];
        if mid_value == target {
            return Some(mid as u32);
        }

        if target < mid_value {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }
    None
}

// 递归实现二分搜索
fn binary_search_recursive_impl(arr: &[u32], target: u32, start: u32, end: u32) -> Option<u32> {
    let mid = (start + end) / 2;
    let mid_val = arr[mid as usize];
    if mid_val == target {
        return Some(mid);
    }
    if start >= end {
        return None;
    }
    if mid_val > target {
        binary_search_recursive_impl(arr, target, start, mid - 1)
    } else {
        binary_search_recursive_impl(arr, target, mid + 1, end)
    }
}

pub fn search_test_1() {
    let target = 6;
    let arr = vec![1, 2, 3, 4, 5, 6];
    let index = sequential_find_for_impl(&arr, target);
    if let Some(v) = index {
        println!("顺序搜索-for循环");
        println!("搜索值{}的index为：{}\n", target, v);
    } else {
        println!("搜索值不在数组中")
    }

    let index = sequential_find_while_impl(&arr, target);
    if let Some(v) = index {
        println!("顺序搜索-while循环");
        println!("搜索值{}的index为：{}\n", target, v);
    } else {
        println!("搜索值不在数组中")
    }

    if let Some(v) = binary_search_while_impl(&arr, target) {
        println!("二分查找-while循环实现");
        println!("搜索值{}的index为：{}\n", target, v);
    } else {
        println!("搜索值不在数组中")
    }

    if let Some(v) = binary_search_recursive_impl(&arr, target, 0, (arr.len() - 1) as u32) {
        println!("二分查找-递归实现");
        println!("搜索值{}的index为：{}\n", target, v);
    } else {
        println!("搜索值不在数组中")
    }

    if let Some(v) = exponential_search(&arr, target) {
        println!("指数查找-基于二分查找递归实现");
        println!("搜索值{}的index为：{}\n", target, v);
    } else {
        println!("搜索值不在数组中")
    }

    if let Some(arr) = any_two_numbers_eq_target(&[2, 1, 4, 3, 3, 5, 6, 7, 8], 6) {
        println!("目标值:{} 为两数之和：{:?}\n", 6, arr)
    }

    if let Some(v) = interpolation_search(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 7) {
        println!("插值查找-while循环实现");
        println!("搜索值{}的index为：{}\n", 7, v);
    } else {
        println!("搜索值不在数组中")
    }
}

// 内插查找 - 二分查找的变形
fn interpolation_search(nums: &[i32], target: i32) -> Option<i32> {
    if nums.is_empty() {
        return None;
    }
    let mut low = 0 as usize;
    let mut high = nums.len() - 1;
    while low <= high {
        // 内插法：已知y1,y0,x1,x0,y2求 x2
        // (y1 - y0)    (y2 - y0)
        // --------- = ---------
        // (x1 - x0)    (x2 - x0)
        // 上面可以转化为
        // x2 = x0 + (y2 - y0) * (x1 - x0) / (y1 - y0)
        let mid = low + ((high - low) as f32 / (nums[high] - nums[low]) as f32 * (target - nums[low]) as f32) as usize;
        if nums[mid] == target {
            return Some(mid as i32);
        } else if nums[mid] > target {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    None
}

// 指数查找
fn exponential_search(nums: &[u32], target: u32) -> Option<u32> {
    let size = nums.len();
    if size == 0 {
        return None;
    }
    // 先通过指数增长的方式确定上下界
    let mut high = 1usize;
    while high < size && nums[high] < target {
        high *= 2;
    }
    let low = high / 2;
    let high = size.min(high + 1) as u32;
    binary_search_recursive_impl(nums, target as u32, low as u32, high)
}

// 给出一个数组，从数组中找到两个数字使二者之和为 target
fn any_two_numbers_eq_target(arr: &[i32], target: i32) -> Option<[i32; 2]> {
    let mut map = HashMap::new();
    for (i, num) in arr.iter().enumerate() {
        let diff = target - num;
        if let Some(index) = map.get(&diff) {
            return Some([(*index) as i32, i.try_into().unwrap()]);
        }
        map.insert(num, i);
    }
    None
}