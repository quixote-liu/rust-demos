// https://leetcode.cn/problems/detect-capital/

struct Solution{}

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let start = "a".as_bytes();
        let end = "z".as_bytes(); 
        let is_up = |code:u8| -> bool {
            if code >= start[0] && code <= end[0] {
                return false;
            }
            return true;
        };

        let mut is_all = true;
        let mut is_not_all = true;
        let mut is_first = word.len() != 1;
        let mut index = 0;
        word.as_bytes().iter().for_each(|code| {
            let ok = is_up(*code);
            if ok {
                is_not_all = false;
                if index != 0 {
                    is_first = false;
                }
            } else {
                is_all = false;
            }
            index += 1;
        });

        is_not_all || is_first || is_all
    }
}