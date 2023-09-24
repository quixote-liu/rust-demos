struct Solution{}

impl Solution {
    pub fn balanced_string_split(mut s: String) -> i32 {
        let mut count = 0;
        let mut balance = 0;
        s.chars().into_iter().for_each(|c| {
            match c {
                'R' => balance += 1,
                _ => balance -= 1,
            }
            if balance == 0 {
                count += 1;
            }
        });
        return count;
    }
}
