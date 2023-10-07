// 题目链接：https://leetcode.cn/problems/sorted-merge-lcci/

pub struct Solution{}

impl Solution {
    pub fn merge(a: &mut Vec<i32>, m: i32, b: &mut Vec<i32>, n: i32) {
        let mut is_sort = false;
        for i in m as usize..a.len() {
            let bv = b[i-m as usize];
            if is_sort {
                a[i] = bv;
                continue;
            }
            let mut index = i-1;
            if index.clone() as i32 == -1 {
                a[i] = bv;
                is_sort = true;
                continue;
            }
            loop {
                println!("index = {}", index);
                if a[index] > bv {
                    let temp = a[index];
                    a[index] = bv;
                    a[index+1] = temp;
                } else {
                    if index == i-1 {
                        a[i] = bv;
                        is_sort = true;
                    }
                    break;
                }
                if index == 0 {
                    break;
                }
                index -= 1;
            }
        }
    }
}
