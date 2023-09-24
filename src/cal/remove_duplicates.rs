struct Solution{}

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut result = Vec::new();
        s.bytes().for_each(|c| {
            match result.last() {
                Some(&a) => {
                    if a == c {
                        result.pop();
                    } else {
                        result.push(c);
                    }
                },
                _ => result.push(c),
            }
        });
        String::from_utf8(result).unwrap()
    }
}