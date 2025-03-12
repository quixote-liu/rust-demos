// 1337. 矩阵中战斗力最弱的 K 行
// 简单
// 相关标签
// 相关企业
// 提示
// 给你一个大小为 m * n 的矩阵 mat，矩阵由若干军人和平民组成，分别用 1 和 0 表示。

// 请你返回矩阵中战斗力最弱的 k 行的索引，按从最弱到最强排序。

// 如果第 i 行的军人数量少于第 j 行，或者两行军人数量相同但 i 小于 j，那么我们认为第 i 行的战斗力比第 j 行弱。

// 军人 总是 排在一行中的靠前位置，也就是说 1 总是出现在 0 之前。

 

// 示例 1：

// 输入：mat = 
// [[1,1,0,0,0],
//  [1,1,1,1,0],
//  [1,0,0,0,0],
//  [1,1,0,0,0],
//  [1,1,1,1,1]], 
// k = 3
// 输出：[2,0,3]
// 解释：
// 每行中的军人数目：
// 行 0 -> 2 
// 行 1 -> 4 
// 行 2 -> 1 
// 行 3 -> 2 
// 行 4 -> 5 
// 从最弱到最强对这些行排序后得到 [2,0,3,1,4]
// 示例 2：

// 输入：mat = 
// [[1,0,0,0],
//  [1,1,1,1],
//  [1,0,0,0],
//  [1,0,0,0]], 
// k = 2
// 输出：[0,2]
// 解释： 
// 每行中的军人数目：
// 行 0 -> 1 
// 行 1 -> 4 
// 行 2 -> 1 
// 行 3 -> 1 
// 从最弱到最强对这些行排序后得到 [0,2,3,1]

struct Solution;

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        struct RowInfo(i32, i32);
        let mut rows: Vec<RowInfo> = Vec::new();
        for i in 0..mat.len() {
            let r = &mat[i];
            let mut p_count = 0;
            for j in 0..r.len() {
                if r[j] == 1 {
                    p_count += 1;
                    continue;
                }
                break;
            }
            rows.push(RowInfo(p_count, i as i32));
        }
        rows.sort_by(|a, b| {
            if (b.0 > a.0) || (b.0 == a.0 && b.1 > a.1) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });
        let mut res = Vec::new();
        for i in 0..k {
            let r = &rows[i as usize];
            res.push(r.1);
        }

        res
    }
}