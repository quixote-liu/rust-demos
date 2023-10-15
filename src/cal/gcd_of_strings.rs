struct Solution{}

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if str1.eq(&str2) { return str1 };
        let mut model = String::new();
        let min_len = std::cmp::min(str1.len(), str2.len());
        for i in 0..min_len {
            let model1 = str1.get(0..i+1).unwrap();  
            let model2 = str2.get(0..i+1).unwrap();
            if model1 != model2 {
                return model;
            }
            
            let mut build_str = String::new();
            let mut str_ok = (false, false); 
            while build_str.len() < str1.len() || build_str.len() < str2.len() {
                build_str.push_str(&model1);
                if build_str.eq(&str1) {
                    str_ok.0 = true;
                }
                if build_str.eq(&str2) {
                    str_ok.1 = true;
                }
            }
            if str_ok.0 && str_ok.1 {
                model = String::from(model1);
            }
        };
        return model;
    }

    pub fn gcd_of_strings_2(str1: String, str2: String) -> String {
        "".to_string()
    }
}
