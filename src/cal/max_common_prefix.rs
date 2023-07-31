use std::cmp;

// 编写一个函数来查找字符串数组中的最长公共前缀。
// 如果不存在公共前缀，返回空字符串 ""。

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.len() == 1 {
        return strs[0].clone();
    }
    let mut common_prefix = String::from("");
    let compared = (&strs[0]).as_bytes();
    for i in 1..strs.len() {
        let mut new_common_prefix = String::from("");
        let current = (&strs[i]).as_bytes();
        let min_len = cmp::min(current.len(), compared.len());
        for index in 0..min_len {
            if compared[index] == current[index] {
                let var = [compared[index]];
                new_common_prefix.push_str(std::str::from_utf8(&var).unwrap());
            } else {
                break;
            }
        }
        if (common_prefix.len() > new_common_prefix.len()) || (common_prefix.len() == 0 && new_common_prefix.len() != 0) {
            common_prefix = new_common_prefix;
        }
        if common_prefix.len() == 0 {
            break;
        }
    }
    return common_prefix;
}
