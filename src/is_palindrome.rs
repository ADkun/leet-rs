pub fn is_palindrome(x: i32) -> bool {
    let s = x.to_string();
    let len = s.len();
    for index in 0..(len / 2) {
        if s.chars().nth(index).unwrap() != s.chars().nth(len - index - 1).unwrap() {
            return false;
        }
    }
    true
}

pub fn is_palindrome_optimize(x: i32) -> bool {
    if x < 0 || (x % 10 == 0 && x != 0) {
        return false;
    }

    let mut ori = x;
    let mut rev = 0;
    while ori > rev {
        rev = rev * 10 + ori % 10;
        ori /= 10;
    }

    ori == rev || ori == rev / 10
}
