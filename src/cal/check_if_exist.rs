// https://leetcode.cn/problems/check-if-n-and-its-double-exist/description/
// 1346. 检查整数及其两倍数是否存在
// 简单
// 相关标签
// 相关企业
// 提示
// 给你一个整数数组 arr，请你检查是否存在两个整数 N 和 M，满足 N 是 M 的两倍（即，N = 2 * M）。

// 更正式地，检查是否存在两个下标 i 和 j 满足：

// i != j
// 0 <= i, j < arr.length
// arr[i] == 2 * arr[j]
 

// 示例 1：

// 输入：arr = [10,2,5,3]
// 输出：true
// 解释：N = 10 是 M = 5 的两倍，即 10 = 2 * 5 。
// 示例 2：

// 输入：arr = [7,1,14,11]
// 输出：true
// 解释：N = 14 是 M = 7 的两倍，即 14 = 2 * 7 。
// 示例 3：

// 输入：arr = [3,1,7,11]
// 输出：false
// 解释：在该情况下不存在 N 和 M 满足 N = 2 * M 。
 

// 提示：

// 2 <= arr.length <= 500
// -10^3 <= arr[i] <= 10^3

struct Solution;

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut double = std::collections::HashMap::new();
        for (i, e) in arr.iter().enumerate() {
            double.insert(2*e, i);
        }
        for (i, e) in arr.iter().enumerate() {
            if let Some(ii) = double.get(e) {
                if i != *ii {
                    return true
                }
            }
        }
        false
    }
}