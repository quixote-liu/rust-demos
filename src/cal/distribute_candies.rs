// 题目：https://leetcode.cn/problems/distribute-candies-to-people/

use core::num;

struct Solution {}

impl Solution {
    pub fn distribute_candies(mut candies: i32, num_people: i32) -> Vec<i32> {
        let mut given = 0;
        let mut result = Vec::new();
        result.resize(num_people as usize, 0);
        let mut index = 0;
        loop {
            if index as i32 == num_people {
                index = 0
            }

            given += 1;
            let res = candies - given;
            if res < 0 {
                result[index] += candies;
                break;
            }
            result[index] += given;
            candies -= given;
            if candies == 0 {
                break;
            }
            index += 1;
        };
        return result;
    } 

}