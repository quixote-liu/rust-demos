// https://leetcode.cn/problems/kth-missing-positive-number/description/

struct Solution;

impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        if arr.len() == 0 {
            return 0
        }
        let mut res: i32 = 0;
        let mut arr_index: usize = 0;
        for _ in 0..k {
            res += 1;
            loop {
                if arr_index < arr.len() {
                    if res == arr[arr_index] {
                        res += 1;
                        arr_index += 1;
                        continue;
                    }
                }
                break;
            }
        }
        res
    }
}