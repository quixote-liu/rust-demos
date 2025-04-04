// 1480. 一维数组的动态和
// 简单
// 相关标签
// 相关企业
// 提示
// 给你一个数组 nums 。数组「动态和」的计算公式为：runningSum[i] = sum(nums[0]…nums[i]) 。

// 请返回 nums 的动态和。

 

// 示例 1：

// 输入：nums = [1,2,3,4]
// 输出：[1,3,6,10]
// 解释：动态和计算过程为 [1, 1+2, 1+2+3, 1+2+3+4] 。
// 示例 2：

// 输入：nums = [1,1,1,1,1]
// 输出：[1,2,3,4,5]
// 解释：动态和计算过程为 [1, 1+1, 1+1+1, 1+1+1+1, 1+1+1+1+1] 。
// 示例 3：

// 输入：nums = [3,1,2,10,1]
// 输出：[3,4,6,16,17]
 

// 提示：

// 1 <= nums.length <= 1000
// -10^6 <= nums[i] <= 10^6

struct Solution;

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::with_capacity(nums.len());
        let mut cur_total = 0;
        nums.iter().for_each(|e| {
            cur_total += *e;
            res.push(cur_total);
        });
        res
    }
}