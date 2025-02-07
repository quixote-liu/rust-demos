// 1422. 分割字符串的最大得分
// 简单
// 相关标签
// 相关企业
// 提示
// 给你一个由若干 0 和 1 组成的字符串 s ，请你计算并返回将该字符串分割成两个 非空 子字符串（即 左 子字符串和 右 子字符串）所能获得的最大得分。

// 「分割字符串的得分」为 左 子字符串中 0 的数量加上 右 子字符串中 1 的数量。

 

// 示例 1：

// 输入：s = "011101"
// 输出：5 
// 解释：
// 将字符串 s 划分为两个非空子字符串的可行方案有：
// 左子字符串 = "0" 且 右子字符串 = "11101"，得分 = 1 + 4 = 5 
// 左子字符串 = "01" 且 右子字符串 = "1101"，得分 = 1 + 3 = 4 
// 左子字符串 = "011" 且 右子字符串 = "101"，得分 = 1 + 2 = 3 
// 左子字符串 = "0111" 且 右子字符串 = "01"，得分 = 1 + 1 = 2 
// 左子字符串 = "01110" 且 右子字符串 = "1"，得分 = 2 + 1 = 3
// 示例 2：

// 输入：s = "00111"
// 输出：5
// 解释：当 左子字符串 = "00" 且 右子字符串 = "111" 时，我们得到最大得分 = 2 + 3 = 5
// 示例 3：

// 输入：s = "1111"
// 输出：3

struct Solution;

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let mut res = 0;
        let bytes = s.as_bytes();
        for i in 1..bytes.len() {
            let (left, right) = bytes.split_at(i);  
            let val = Self::count_left_str(left) + Self::count_right_str(right);
            if val > res {
                res = val;
            }
        };
        res
    }

    fn count_left_str(v: &[u8]) -> i32 {
        let mut res = 0;
        v.iter().for_each(|e|{
            if *e == b'0' {
                res += 1;
            }
        });
        res
    }

    fn count_right_str(v: &[u8]) -> i32 {
        let mut res = 0;
        v.iter().for_each(|e| {
            if *e == b'1' {
                res += 1;
            }
        });
        res
    }
}