use std::cmp::min;
// 动态规划
use std::collections::{BTreeMap, HashMap};

// 找零-基于递归的解决方案
// 当找零金额过大时，无法计算出结果
// 太多重复计算，无法记住重复组合
fn rec_mc1(cashes: &[u32], amount: u32) -> u32 {
    // 全用1元找零 币数
    let mut min_cashes = amount;
    // 如果找零金额列表正好包含要找零的金额
    if cashes.contains(&amount) {
        return 1;
    } else {
        // 过滤出合适的找零金额列表（面值>找零金额的不要）
        for c in cashes.iter()
            .filter(|&c| *c <= amount)
            .collect::<Vec<&u32>>() {
            // amount - c 表示用了一张面额为c的纸币
            // 所以要+1
            let num_cashes = 1 + rec_mc1(&cashes, amount - c);
            // 如果得出的结果比 min_cashes小就更新（表示用了更少的找零数量）
            if num_cashes < min_cashes {
                min_cashes = num_cashes;
            }
        }
        min_cashes
    }
}

pub fn dp_test_1() {
    let cashes = [1, 5, 10, 20, 50];
    let amount = 31 as u32;
    let cashes_num = rec_mc1(&cashes, amount);
    println!("找零数量：{}", cashes_num)
}

// 空间换时间例子
fn rec_mc2(cashes: &[u32], amount: u32, min_cashes_map: &mut HashMap<u32, u32>) -> u32 {
    // 全用1元找零数量
    let mut current_min_cashes_num = amount;
    if cashes.contains(&amount) {
        min_cashes_map.insert(amount, 1);
        // 收集 - 当前找零值相同的币种
        return 1;
    }
    // 检查是否有缓存
    match min_cashes_map.get(&amount) {
        // 如果没有计算过，就计算
        None => {
            // 过滤出合适的找零金额列表（面值>找零金额的不要）
            for cash in cashes.iter()
                .filter(|&c| *c <= amount)
                .collect::<Vec<&u32>>() {
                // amount - c 表示用了依仗面额为c的纸币
                let cash = *cash;
                let next_amount = amount - cash;
                // 所以要+1
                let cashes_num = 1 + rec_mc2(cashes, next_amount, min_cashes_map);
                // 如果得出的结果比 min_cashes小就更新（表示用了更少的找零数量）
                if cashes_num < current_min_cashes_num {
                    current_min_cashes_num = cashes_num;
                    // 更新缓存
                    min_cashes_map.insert(amount, current_min_cashes_num);
                }
            }
        }
        // 如果已经计算过，就直接取缓存
        Some(v) => {
            return v.clone();
        }
    }
    current_min_cashes_num
}

pub fn cal_change_num(cashes: &[u32], mut amount: u32, change_list: &mut Vec<u32>) -> u32 {
    while amount > 0 {
        let max = cashes.iter().filter(|&item| *item <= amount).max();
        if let Some(max) = max {
            change_list.push(*max);
            amount -= *max;
        }
    }
    change_list.len() as u32
}

pub fn dp_test_2() {
    let amount = 81 as u32;
    let cashes: [u32; 5] = [1, 5, 10, 20, 50];
    let mut min_cashes = HashMap::new();
    let mut change_list: Vec<u32> = vec![];
    let result = rec_mc2(&cashes, amount, &mut min_cashes);
    println!("递归实现 - 找零数量：{}", result);
    let result = cal_change_num(&cashes, amount, &mut change_list);
    println!("循环实现 - 找零数量：{}", result);
    println!("找了哪些零：{:?}", change_list);
}


pub fn dp_rec_mc(cashes: &[u32], amount: u32, cash_map: &mut BTreeMap<u32, u32>) -> u32 {
    // 小问题推导大问题
    // 从1到求解金额的所有情况
    for current_amount in 1..=amount {
        // 全用1元找零的情况
        let mut min_times = current_amount;
        // 过滤出来可用的找零金额
        for cash in cashes.iter()
            .filter(|&cash| *cash <= current_amount)
            .collect::<Vec<&u32>>() {
            // 假设此时当前金额为 6，可用找零分别为1和5
            // index = 6 - 5 -> 此时index为1 -> 1代表着当找零为1元时需要找几次
            // index = 6 - 1 -> 此时index为5 -> 5代表着当找零为5元时需要找几次
            let index = current_amount - cash;
            // 两次循环分别计算当前的找零次数
            let current_cash_times = 1 + match cash_map.get(&index) {
                Some(v) => *v,
                None => 0,
            };
            // 如果当前找零次数<最小找零次数，就更新
            if current_cash_times < min_times {
                min_times = current_cash_times;
            }
        }
        // 将当前金额对应的最小找零次数写入缓存
        cash_map.insert(current_amount, min_times);
    }
    // 从缓存中找到求解金额的找零次数
    *cash_map.get(&amount).unwrap()
}

fn dp_rec_mc_show(cashes: &[u32], amount: u32, cash_map: &mut BTreeMap<u32, u32>, cashes_used: &mut [u32]) -> u32 {
    for current_amount in 1..=amount {
        let mut min_cash_times = current_amount;
        let mut used_cash = 1;
        for cash in cashes.iter()
            .filter(|&cash| *cash <= current_amount)
            .collect::<Vec<&u32>>() {
            let index = current_amount - cash;
            let current_cash_times = 1 + match cash_map.get(&index) {
                Some(v) => *v,
                None => 0
            };
            if current_cash_times < min_cash_times {
                min_cash_times = current_cash_times;
                used_cash = *cash;
            }
        }
        cash_map.insert(current_amount, min_cash_times);
        cashes_used[current_amount as usize] = used_cash;
    }
    *cash_map.get(&amount).unwrap()
}

fn print_cashes(cashes_used: &[u32], mut amount: u32) {
    while amount > 0 {
        let current = cashes_used[amount as usize];
        print!("{} ", current);
        amount -= current;
    }
    println!()
}

pub fn dp_test_3() {
    const AMOUNT: u32 = 81u32;
    let cashes = [1, 5, 10, 20, 50];
    let mut cash_map = BTreeMap::new();
    let mut cash_list = [0; (AMOUNT + 1) as usize];
    let cash_num = dp_rec_mc(&cashes, AMOUNT, &mut cash_map);
    let cash_num = dp_rec_mc_show(&cashes, AMOUNT, &mut cash_map, &mut cash_list);
    println!("找零￥{} 需要 {} 次", AMOUNT, cash_num);
    println!("{:?}", cash_map);
    println!("{:?}", cash_list);
    print_cashes(&cash_list, AMOUNT);
}

