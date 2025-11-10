struct Solution;

impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let ns = n.to_string();
        let mut nums = Vec::new();
        ns.chars().into_iter().for_each(|e| {
            if let Some(n) = e.to_digit(10) {
                nums.push(n as i32);
            }
        });
        
        let mut sum = 0;
        let mut mul = 1;
        nums.iter().for_each(|e| {
           sum += e;
           mul *= e;
        });

        mul - sum
    }
}