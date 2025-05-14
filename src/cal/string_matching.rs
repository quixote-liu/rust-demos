// https://leetcode.cn/problems/string-matching-in-an-array/
struct Solution;

impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let mut res = Vec::new();
        for i in 0..words.len() {
            for j in 0..words.len() {
                if j == i || words[i].len() > words[j].len() {
                    continue
                }
                println!("words[{}]= {}, words[{}]={}", i, words[i], j, words[j]);
                let bytesi = words[i].as_bytes();
                let bytesj = words[j].as_bytes();
                let mut start = 0;
                let mut is_sub = false;
                loop {
                    let end = start + bytesi.len() - 1;
                    if end >= bytesj.len() {
                        break;
                    }
                    let mut is_match = true;
                    for n in start..end+1 {
                        if bytesj[n] != bytesi[n-start] {
                            is_match = false;
                            break;
                        }
                    }
                    if is_match {
                        res.push(words[i].clone());
                        is_sub = true;
                        break;
                    }
                    start += 1;
                }
                if is_sub {
                    break;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo() {
        let param: Vec<String> = vec!["leetcoder".to_string(), "leetcode".to_string(), "od".to_string(), "hamlet".to_string(), "am".to_string()];
        let res = Solution::string_matching(param);
        print!("res = {:?}", res);
    }
}