#![allow(dead_code)]

/// 方法一：纵向扫描
/// 1. 遍历数组，获取最短字符串长度
/// 2. 遍历最短字符串长度，比较数组中每一个字符串的对应的字符
/// 3. 如果该字符相同，将其加入结果中，继续比较下一个字符。
/// 4. 如果该字符不同，结束被返回。
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut result = String::new();

    let mut shortest_len = std::usize::MAX;
    for s in strs.iter() {
        if s.len() < shortest_len {
            shortest_len = s.len();
        }
    }

    for i in 0..shortest_len {
        let first_char = strs[0].chars().nth(i).unwrap();
        for (j, s) in strs.iter().enumerate() {
            if j == 0 {
                continue;
            }

            if s.chars().nth(i).unwrap() != first_char {
                return result;
            }
        }
        result.push(first_char);
    }

    result
}

/// 方法二：横向扫描
/// 1. 将数组第一个字符串视作前缀
/// 2. 将当前前缀与下一个字符串比较，得出两个字符串的前缀
/// 3. 两两比较完所有字符串，所得的前缀就是最长公共前缀。
pub fn longest_common_prefix_landscape(strs: Vec<String>) -> String {
    if strs.len() == 0 {
        return String::new();
    }

    let mut prefix = strs[0].clone();

    for i in 1..strs.len() {
        prefix = longest_common_prefix_string(&prefix, &strs[i]);
        if prefix.is_empty() {
            break;
        }
    }

    prefix
}

fn longest_common_prefix_string(s1: &str, s2: &str) -> String {
    let length = s1.len().min(s2.len());
    let mut index = 0;
    while index < length && s1.chars().nth(index) == s2.chars().nth(index) {
        index += 1;
    }
    s1[0..index].to_string()
}

/// 方法三：二分法
/// 1. 获取最短字符串长度
/// 2. 进行二分扫描，如果当前二分的是最短字符串，则将二分位置增加。反之，减少。
/// 3. 当双指针重合，说明找完了，返回0..low的结果。
pub fn longest_common_prefix_binary(strs: Vec<String>) -> String {
    if strs.len() == 0 {
        return String::new();
    }

    let mut min_len = std::usize::MAX;
    for s in strs.iter() {
        if s.len() < min_len {
            min_len = s.len();
        }
    }

    let mut low = 0;
    let mut high = min_len;
    while low < high {
        let mid = (high - low + 1) / 2 + low;

        if is_common_prefix(&strs, mid) {
            low = mid;
        } else {
            high = mid - 1;
        }
    }

    strs[0][0..low].to_string()
}

fn is_common_prefix(strs: &Vec<String>, length: usize) -> bool {
    let first_str = &strs[0][0..length];
    for i in 1..strs.len() {
        let temp_str = &strs[i];
        for j in 0..length {
            if first_str.chars().nth(j).unwrap() != temp_str.chars().nth(j).unwrap() {
                return false;
            }
        }
    }
    true
}

/// 方法四：分治
/// 1. 不断二分整个数组，获得左右两边的最长公共前缀
/// 2. 把左右两边的最长公共前缀再进行比较，获取这最长公共前缀。
pub fn longest_common_prefix_devide(strs: Vec<String>) -> String {
    if strs.len() == 0 {
        return String::new();
    }

    longest_common_prefix_recursive(&strs, 0, strs.len() - 1)
}

fn longest_common_prefix_recursive(strs: &Vec<String>, start: usize, end: usize) -> String {
    if start == end {
        return strs[start].clone();
    }

    let mid = (start + end) / 2;
    let left = longest_common_prefix_recursive(strs, start, mid);
    let right = longest_common_prefix_recursive(strs, mid + 1, end);
    common_prefix(&left, &right)
}

fn common_prefix(left: &String, right: &String) -> String {
    let min_len = left.len().min(right.len());
    for i in 0..min_len {
        if left.chars().nth(i).unwrap() != right.chars().nth(i).unwrap() {
            return left[0..i].to_string();
        }
    }
    left[0..min_len].to_string()
}
