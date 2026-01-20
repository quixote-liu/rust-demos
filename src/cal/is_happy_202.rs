struct Solution;

impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        let mut all_nums = std::collections::HashMap::new();
        loop {
            let mut new_n = 0;
            let eles = Self::split_num(n);
            eles.iter().for_each(|e|{
                new_n += (*e) * (*e);
            });
            if new_n == 1 {
                return true
            } else {
                match all_nums.get(&new_n) {
                    Some(_) => return false,
                    None => {
                        all_nums.insert(new_n, true);
                        n = new_n;
                    },
                }
            }
        }
    }

    fn split_num(mut n: i32) -> Vec<i32> {
        let mut res = Vec::new();
        while n > 0 {
            let v = n % 10;
            res.push(v);
            n = n / 10;
        }
        res
    }
}