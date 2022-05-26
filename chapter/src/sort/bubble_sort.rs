

// 冒泡排序-for实现 性能为 O(n^2)
fn bubble_sort_for_impl(nums: &mut [i32]) {
    for i in 1..nums.len() {
        for j in 0..nums.len() - i {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1)
            }
        }
    }
}


// 冒泡排序-while实现
fn bubble_sort_while_impl(nums: &mut [i32]) {
    let mut len = nums.len() - 1;
    while len > 0 {
        for i in 0..len {
            if nums[i] > nums[i + 1] {
                nums.swap(i, i + 1);
            }
        }
        len -= 1;
    }
}

// 冒泡排序改造 - 如果数组有序直接就不再后续比较
fn bubble_sort_pro(nums: &mut [i32]) {
    let mut len = nums.len() - 1;
    let mut compare = true;
    while len > 0 && compare {
        compare = false;
        for i in 0..len {
            if nums[i] > nums[i + 1] {
                compare = true;
                nums.swap(i, i + 1);
            }
        }
        len -= 1;
    }
}

// 冒泡排序-改造 - 鸡尾酒排序 复杂度O(n^2) 已排序则接近O(n)
fn bubble_sort_cocktail(nums: &mut [i32]) {
    let mut bubble = true;
    let len = nums.len();
    for i in 0..(len / 2) {
        if bubble {
            bubble = false;
            // 从左到右
            for j in i..(len - i - 1) {
                if nums[j] > nums[j + 1] {
                    nums.swap(j, j + 1);
                    bubble = true;
                }
            }

            // 从右到左
            for j in (i + 1..(len - i - 1)).rev() {
                if nums[j] < nums[j - 1] {
                    nums.swap(j - 1, j);
                    bubble = true;
                }
            }
        } else {
            break;
        }
    };
}


pub fn sort_test_1() {
    let mut arr = vec![5, 4, 3, 1, 6, 2, 1, 3, 9, 12, 43];
    bubble_sort_while_impl(&mut arr);
    bubble_sort_for_impl(&mut arr);
    bubble_sort_pro(&mut arr);
    bubble_sort_cocktail(&mut arr);
    println!("冒泡排序后：{:?}", arr);
}