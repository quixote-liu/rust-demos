struct Solution;

impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        if arr.len() < 3 {
            return false;
        }

        for i in 0..arr.len()-2 {
            let exist = arr[i] % 2 != 0 && arr[i+1] % 2 != 0 && arr[i+2] % 2 != 0;
            if exist {
                return true;
            }
        }
        
        false
    }
}