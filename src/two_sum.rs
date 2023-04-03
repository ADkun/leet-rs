use std::collections::HashMap;

pub fn two_sum_hash(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for i in 0..nums.len() {
        let diff = target - nums[i];
        if map.contains_key(&diff) {
            return vec![*map.get(&diff).unwrap(), i as i32];
        }
        map.insert(nums[i], i as i32);
    }
    return vec![];
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut res = Vec::new();
    'x: for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                res.push(i as i32);
                res.push(j as i32);
                break 'x;
            }
        }
    }
    return res;
}
