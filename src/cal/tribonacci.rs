// https://leetcode.cn/problems/n-th-tribonacci-number/submissions/
// 第n个斐波那契数

struct Solution{}

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut res = 0;
        if n == 1 || n == 2 {
            return 1;
        }
        let mut t0 = 0;
        let mut t1 = 1;
        let mut t2 = 1;
        for i in 0..n-2 {
            res = t0 + t1 + t2;
            t0 = t1;
            t1 = t2;
            t2 = res;
        };
        return res;
    }
}