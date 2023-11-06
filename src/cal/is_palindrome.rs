// https://leetcode.cn/problems/palindrome-number/

struct Solution{}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut step = 1;
        let mut xx = x;
        let mut nums = Vec::new();
        while xx != 0 {
            let temp = xx % 10;
            nums.push(temp);
            xx /= 10;
            step *= 10;
        }
        let mut res = 0;
        nums.into_iter().for_each(|e| {
            step /= 10;
            res += e * step;
        });
        res == x
    }

    // pub fn is_palindrome(x: i32) -> bool {
    //     if x < 0 {
    //         return false;
    //     }
    //     let xx = x.to_string();
    //     let chars = xx.as_bytes();
    //     let mut start = 0;
    //     let mut end = xx.len() - 1;
    //     while start < end {
    //         if chars[start] != chars[end] {
    //             return false;
    //         } else {
    //             start += 1;
    //             end -= 1;
    //             continue;
    //         }
    //     }
    //     true
    // }
}