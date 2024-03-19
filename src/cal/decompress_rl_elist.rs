// https://leetcode.cn/problems/decompress-run-length-encoded-list/

struct Solution{}

impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let mut nums_res = Vec::new();
        let mut index = 0;
        loop {
            match nums.get(index) {
                Some(pref) => {
                    match nums.get(index+1) {
                        Some(val) => {
                            for _ in 0..*pref {
                                nums_res.push(*val);
                            }
                        },
                        None => {return nums_res}
                    }
                },
                None => {return nums_res},
            }
            index += 2;
        }
    }
}