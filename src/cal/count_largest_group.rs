// https://leetcode.cn/problems/count-largest-group/description/
// 1399. 统计最大组的数目
// 简单
// 相关标签
// 相关企业
// 提示
// 给你一个整数 n 。请你先求出从 1 到 n 的每个整数 10 进制表示下的数位和（每一位上的数字相加），然后把数位和相等的数字放到同一个组中。

// 请你统计每个组中的数字数目，并返回数字数目并列最多的组有多少个。

 

// 示例 1：

// 输入：n = 13
// 输出：4
// 解释：总共有 9 个组，将 1 到 13 按数位求和后这些组分别是：
// [1,10]，[2,11]，[3,12]，[4,13]，[5]，[6]，[7]，[8]，[9]。总共有 4 个组拥有的数字并列最多。
// 示例 2：

// 输入：n = 2
// 输出：2
// 解释：总共有 2 个大小为 1 的组 [1]，[2]。
// 示例 3：

// 输入：n = 15
// 输出：6
// 示例 4：

// 输入：n = 24
// 输出：5
 

// 提示：

// 1 <= n <= 10^4

struct Solution;

impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let count = |mut num: i32| -> i32 {
            let mut res = 0;
            loop {
                if num >= 10 {
                    res += num % 10;
                    num = num / 10;
                } else {
                    res += num;
                    break res;
                }
            }
        };

        use std::collections::HashMap;
        let mut group_map: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut max_group_len = 0;
        for i in 1..n+1 {
            let mut c = 0;
            if i < 10 {
                c = i;
            } else {
                c = count(i);
            }
            let group_len: usize;
            if let Some(v) = group_map.get_mut(&c) {
                v.push(i);
                group_len = v.len();
            } else {
                group_map.insert(c, Vec::from([i]));
                group_len = 1;
            }
            if max_group_len < group_len {
                max_group_len = group_len;
            }
        }

        let mut res = 0;
        group_map.iter().for_each(|e| {
            if e.1.len() == max_group_len {
                res += 1;
            }
        });

        res as i32
    }
}