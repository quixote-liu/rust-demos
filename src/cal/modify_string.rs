struct Solution;

impl Solution {
    pub fn modify_string(s: String) -> String {
        let s_len = s.len() as i32;
        let mut sb = s.as_bytes().to_vec();
        let mut pre_index: i32 = -1;
        let mut cur_index = 0;
        let mut next_index = 1;
        loop {
            if cur_index >= s.len() {
                break String::from_utf8(sb).unwrap();
            }
            let cur = sb[cur_index as usize] as char;
            if cur == '?' {
                let mut pre_val: Option<char> = None;
                if pre_index != -1 && pre_index < s_len {
                    pre_val = Some(sb[pre_index as usize] as char);
                }
    
                let mut next_val: Option<char> = None;
                if next_index < s_len {
                    next_val = Some(sb[next_index as usize] as char);
                }

                // 开始替换该字符
                let mut replace_char = 'a' as u8;
                loop {
                    if let Some(pv) = pre_val {
                        if replace_char as char == pv {
                            replace_char += 1;
                            continue;
                        }
                    }
                    if let Some(nv) = next_val {
                        if nv != '?' && replace_char as char == nv {
                            replace_char += 1;
                            continue
                        }
                    }
                    sb[cur_index] = replace_char;
                    break;
                }

            }
            
            pre_index += 1;
            cur_index += 1;
            next_index += 1;
        }
    }
}