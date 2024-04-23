// https://leetcode.cn/problems/find-n-unique-integers-sum-up-to-zero/

struct Solution{}

impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        if n <= 0 {
            return vec![0];
        }
        let mut res = Vec::new();
        let mut index = n;
        let mut num = 1;
        while index > 1 {
            res.append(&mut vec![num, -num]);
            num += 1;
            index -= 2;
        }
        if index == 1 {
            res.push(0);
        }
        res
    }
}