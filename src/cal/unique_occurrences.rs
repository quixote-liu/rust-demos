pub struct Solution{}

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let res = true;
        let mut nums = std::collections::HashMap::new();
        arr.iter().for_each(|val| {
            nums.entry(val).and_modify(|collector| *collector += 1).or_insert(1);
        });

        let mut times = std::collections::HashMap::new();
        for (_key, t) in nums.iter() {
            let mut is_dup = false;
            times.entry(t).and_modify(|_e| is_dup = true).or_insert(true);
            if is_dup {break;}
        }
        return res;
    }
}