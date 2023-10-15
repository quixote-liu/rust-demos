struct Solution{}

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        "".to_string()
    }
}

fn helper(str1: String, str2: String) -> String {
    let str1_len = str1.len();
    let str2_len = str2.len();
    if str1_len % str2_len == 0 {
        let times = str1_len / str2_len;
        let mut s = String::new();
        for i in 0..times {
            s.push_str(&str2);
        }
        if s.eq(&str1) {
            return str2;
        }
    };
    "".to_string()
}
