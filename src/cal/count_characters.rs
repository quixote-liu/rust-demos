struct Solution{}

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut chars_map = std::collections::HashMap::new();
        for c in chars.bytes() {
            if let Some(val) = chars_map.get(&c) {
                chars_map.insert(c, val+1);
            } else {
                chars_map.insert(c, 1);
            }
        };
        let mut count = 0;
        for word in words.iter() {
            let mut chars_map = chars_map.clone();
            let mut word_ok = true;
            for c in word.bytes() {
                if !word_ok {
                    break;
                }
                if let Some(repeats) = chars_map.get(&c) {
                    let new_repeats = repeats - 1;
                    if new_repeats < 0 {
                        word_ok = false;
                        break;
                    }
                    chars_map.insert(c, new_repeats);
                } else {
                    word_ok = false;
                    break;
                }
            };
            if word_ok {
                count += word.len() as i32;
            }
        };
        return count;
    }
}