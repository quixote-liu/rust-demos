struct Solution{}

impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let metric = (arr.len() as f32) * 0.25;
        let mut last_ele = arr[0].clone();
        let mut dup_num = 1;
        for i in 1..arr.len() {
            if last_ele == arr[i] {
                dup_num += 1;
            } else {
                if (dup_num as f32) > metric {
                    return last_ele;
                }
                dup_num = 1;
                last_ele = arr[i].clone();
            }
        }
        last_ele
    }
}