// https://leetcode.cn/problems/find-lucky-integer-in-an-array/
struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut m: HashMap<i32, i32> = HashMap::new();
        arr.iter().for_each(|e|{
            if let Some(v) = m.get(e) {
                m.insert(*e, v + 1);
            } else {
                m.insert(*e, 1);
            }
        });
        let mut lucky_val: Option<i32> = None;
        m.into_iter().for_each(|item| {
            if item.0 == item.1 {
                match lucky_val {
                    Some(lv) => {
                        if lv < item.0 {
                            lucky_val = Some(item.0);
                        }
                    },
                    None => lucky_val = Some(item.0),
                }
            }
        });
        if lucky_val.is_none() {
            return -1;
        }
        return lucky_val.unwrap();
    }
}