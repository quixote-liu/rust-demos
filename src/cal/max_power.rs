// 1446. 连续字符
// 简单
// 相关标签
// 相关企业
// 提示
// 给你一个字符串 s ，字符串的「能量」定义为：只包含一种字符的最长非空子字符串的长度。

// 请你返回字符串 s 的 能量。

 

// 示例 1：

// 输入：s = "leetcode"
// 输出：2
// 解释：子字符串 "ee" 长度为 2 ，只包含字符 'e' 。
// 示例 2：

// 输入：s = "abbcccddddeeeeedcba"
// 输出：5
// 解释：子字符串 "eeeee" 长度为 5 ，只包含字符 'e' 。
 

// 提示：

// 1 <= s.length <= 500
// s 只包含小写英文字母。

struct Solution;

impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut max_len = 0;
        let mut cur_len = 0;
        let mut pre_char: char = ' ';
        s.chars().for_each(|c| {
            if c == ' ' {
                pre_char = c;
                if cur_len > max_len {
                    max_len = cur_len;
                }
                cur_len = 0;
                return
            }
            if c == pre_char {
                cur_len += 1;
            } else {
                if cur_len > max_len {
                    max_len = cur_len;
                }
                pre_char = c;
                cur_len = 1;
            }
        });
        if cur_len > max_len {
            max_len = cur_len;
        }

        max_len
    }
}