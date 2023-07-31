struct Solution{}


impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res = Vec::new();
        for i in 0..nums.len() {
            for j in (i+1)..nums.len() {
                if nums[i] + nums[j] == target {
                    res.push(i as i32);
                    res.push(j as i32);
                    return res;
                }
            }
        }
        res
    }
}

pub fn guess(n: i32) -> i32 {
    1
}

pub fn guess_num(n: i32) -> i32 {
    return helper(0,n);
}

pub fn helper(start:i32, end:i32) -> i32 {
    let mid = (end+start)/2;
    let guess_res = guess(mid);
    if guess_res == 0 {
        return mid;
    }
    if guess_res == -1 {
        helper(start, mid-1);
    } else if guess_res == 1 {
        helper(mid+1, end);
    }
    return 0;
}
