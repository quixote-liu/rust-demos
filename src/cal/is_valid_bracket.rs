
// 给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串 s ，判断字符串是否有效。

// 有效字符串需满足：

// 左括号必须用相同类型的右括号闭合。
// 左括号必须以正确的顺序闭合。
// 每个右括号都有一个对应的相同类型的左括号。

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        let (left1, left2, left3) = ('(', '{', '[');
        let (right1, right2, right3) = (')', '}', ']');
        for b in s.chars().into_iter() {
            if b == left1 || b == left2 || b == left3 {
                stack.push(b);
            } else {
                let compared: char;
                if b == right1 {
                    compared = left1
                } else if b == right2 {
                    compared = left2
                } else if b == right3 {
                    compared = left3
                } else {
                    return false
                }
                if stack.pop() != Some(compared) {
                    return false
                }
            }
        }
        if stack.len() > 0 {
            return false
        }
        return true
    }
}