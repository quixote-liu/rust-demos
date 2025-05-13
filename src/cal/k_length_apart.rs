// https://leetcode.cn/problems/check-if-all-1s-are-at-least-length-k-places-away/

struct Solution;

impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut pre_one_index = -1;
        for i in 0..nums.len() {
            let cur_val = nums[i];
            if cur_val != 1 {
                continue;
            }
            if pre_one_index != -1 && i as i32 - pre_one_index - 1 < k {
                return false;
            }
            pre_one_index = i as i32;
        }
        true
    }
}