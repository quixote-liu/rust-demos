struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn freq_alphabets(s: String) -> String {
        let mut map = HashMap::new();
        for i in 1..27 {
            let mut key = i.to_string();
            let val = 'a' as u8 + i - 1;
            if i <= 9 {
                map.insert(key, val);
            } else {
                key.push('#');
                map.insert(key, val);
            }
        }
        
        let mut result = String::new();
        let mut index = 0;
        loop {
            if index >= s.len() {
                break;
            }
            let ii = index + 2;
            if ii < s.len() {
                let sub_s = s.get(index..ii+1).unwrap();
                if sub_s.ends_with('#') {
                    if let Some(val) = map.get(sub_s) {
                        result.push(*val as char);
                        index += 3;
                        continue;
                    }
                }
            }
            let sub_s = s.get(index..index+1).unwrap();
            if let Some(val) = map.get(sub_s) {
                result.push(*val as char);
            }
            index += 1;
            continue;
        }

        result
    }
}