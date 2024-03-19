// https://leetcode.cn/problems/shift-2d-grid/

struct Solution{}

impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut grid_single: Vec<i32> = Vec::new();
        grid.iter().for_each(|elements| {
            let mut ee = elements.clone();
            grid_single.append(&mut ee);
        });
        let length = grid_single.len();
        if length <= 1 {
            return grid;
        }
        for _ in 0..k {
            let end = grid_single[length-1];
            let mut index: i32 = (length-1) as i32;
            loop {
                if index < 0 || index - 1 < 0 {
                    break;
                }
                grid_single[index as usize] = grid_single[(index-1) as usize];
                index = index - 1;
            }
            grid_single[0] = end
        }
        // 将长数组拆解
        let mut result: Vec<Vec<i32>> = Vec::new();
        grid_single.chunks(grid[0].len()).for_each(|elements| {
            result.push(Vec::from(elements));
        });
        result
    }
}