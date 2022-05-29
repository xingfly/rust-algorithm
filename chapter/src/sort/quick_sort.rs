fn quick_sort(nums: &mut [i32], start: usize, end: usize) {
    if start >= end {
        return;
    }
    let split = partition(nums, start, end);
    quick_sort(nums, start, split - 1);
    quick_sort(nums, split + 1, end);
}

fn partition(nums: &mut [i32], start: usize, end: usize) -> usize {
    // 基准数字（保持原位不动，循环完成后才交换）
    let base = nums[start];
    // 每次要交换的位置（第一次是1）
    let mut index = start + 1;
    for i in index..=end {
        if base > nums[i] {
            // 每次交换过后，更新下次要交换的位置
            nums.swap(i, index);
            index += 1;
        }
    }
    // 最后把选中的基准值放到中间，并返回中间的位置
    nums.swap(start, index - 1);
    index - 1
}

pub fn quick_sort_test_1() {
    let mut nums = [1, 0, 6, 5, 87, 43, 51, 23, 41, 52];
    println!("排序前:{:?}", nums);
    let len = nums.len() - 1;
    quick_sort(&mut nums, 0, len);
    println!("排序后:{:?}", nums);
}

