mod is_palindrome;
mod longest_common_prefix;
mod roman_to_int;
mod two_sum;
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_two_sum() {
        let res = crate::two_sum::two_sum(vec![2, 7, 11, 15], 9);
        let res1 = crate::two_sum::two_sum_hash(vec![2, 7, 11, 15], 9);
        println!("{:?}", res);
        println!("{:?}", res1);
    }

    #[test]
    fn test_is_palindrome() {
        let res = crate::is_palindrome::is_palindrome(121);
        assert_eq!(res, true);
    }

    #[test]
    fn test_roman_to_int() {
        let res = crate::roman_to_int::roman_to_int("III".to_string());
        assert_eq!(res, 3);

        let res = crate::roman_to_int::roman_to_int("IV".to_string());
        assert_eq!(res, 4);

        let res = crate::roman_to_int::roman_to_int("IX".to_string());
        assert_eq!(res, 9);

        let res = crate::roman_to_int::roman_to_int("LVIII".to_string());
        assert_eq!(res, 58);

        let res = crate::roman_to_int::roman_to_int("MCMXCIV".to_string());
        assert_eq!(res, 1994);

        let res = crate::roman_to_int::roman_to_int("MMMCMXCIX".to_string());
        assert_eq!(res, 3999);
    }
}
