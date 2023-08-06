pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
    let mut ans = Vec::new();
    let mut value = 0;
    for i in 0..nums.len() {
        value = value * 2 + nums[i];
        if value % 5 == 0 {
            ans.push(true);
        } else {
            ans.push(false);
        }
        if value >= 10 {
            value = value % 10;
        }
    }
    return ans;
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::cal::prefixes_div_by5;

    #[test]
    fn prefixes_div_by5_test() {
        let nums = vec![0, 1, 1];
        let res = prefixes_div_by5(nums);
        assert_eq!(res, vec![true, false, false]);
    }
}

