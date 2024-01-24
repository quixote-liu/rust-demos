struct Solution{}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let start = 0;
        let end = (nums.len()-1) as i32;
        return Self::search_insert_helper(nums, start, end, target);
    }

    fn search_insert_helper(nums: Vec<i32>, start: i32, end: i32, target: i32) -> i32 {
        if start == end {
            let n = nums[start as usize];
            if n == target {
                return start;
            } else if n > target {
                return if start-1 >= 0 {start-1} else {0};
            } else {
                return start + 1;
            }
        } else {
            let n = nums[end as usize];
            if n == target {
                return end;
            } else if n < target {
                return end + 1;
            }

            let n = nums[start as usize];
            if n == target {
                return start;
            } else if n > target {
                return if start-1 >= 0 {start-1} else {0};
            }

            let mid = (start + end)/2;
            let n = nums[mid as usize];
            if n == target {
                return mid;
            } else if n > target {
                return Self::search_insert_helper(nums, start, mid-1, target);
            } else {
                return Self::search_insert_helper(nums, mid+1, end, target)
            }
        }
    }
}