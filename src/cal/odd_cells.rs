// 题目链接：https://leetcode.cn/problems/cells-with-odd-values-in-a-matrix/

struct Solution{}

impl Solution {
    pub fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut r_append = std::collections::HashMap::new();
        let mut c_append = std::collections::HashMap::new();
        indices.iter().for_each(|e| {
            let ri = e[0];
            if let Some(v) = r_append.get(&ri) {
                r_append.insert(ri, v+1);
            } else {
                r_append.insert(ri, 1);
            }

            let ci = e[1];
            if let Some(v) = c_append.get(&ci) {
                c_append.insert(ci, v+1);
            } else {
                c_append.insert(ci, 1);
            }
        });
        for i in 0..m {
            for j in 0..n {
                let mut ele = 0;
                if let Some(v) = r_append.get(&i) {
                    ele += v;
                }
                if let Some(v) = c_append.get(&j) {
                    ele += v;
                }
                if ele % 2 != 0 {
                    ans += 1;
                }
            }
        };
        ans
    }
}