struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut merged_res = String::with_capacity(word1.len()+word2.len());
        let chars1: Vec<char> = word1.chars().into_iter().collect();
        let chars2: Vec<char> = word2.chars().into_iter().collect();
        let mut index = 0;
        while index < chars1.len() && index < chars2.len() {
            merged_res.push(chars1[index]);
            merged_res.push(chars2[index]);
            index += 1;
        }
        if index < chars1.len() {
            for i in index..chars1.len() {
                merged_res.push(chars1[i]);
            }
        } else if index < chars2.len() {
            for i in index..chars2.len() {
                merged_res.push(chars2[i]);
            }
        }
        return merged_res
    }
}

impl Solution {
    pub fn merge_alternately_v2(word1: String, word2: String) -> String {
        let mut merged_res = String::with_capacity(word1.len()+word2.len());
        let mut chars1 = word1.chars();
        let mut chars2 = word2.chars();
        loop {
            match (chars1.next(), chars2.next()) {
                (Some(c1), Some(c2)) => {
                    merged_res.push(c1);
                    merged_res.push(c2);
                },
                (Some(c1), None) => {
                    merged_res.push(c1);
                },
                (None, Some(c2)) => {
                    merged_res.push(c2);
                }
                (None, None) => break
            }
        }
        merged_res
    }
}