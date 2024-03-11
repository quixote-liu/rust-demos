use std::collections::HashMap;

struct Solution{}

impl Solution {
    pub fn all_cells_dist_order(rows: i32, cols: i32, r_center: i32, c_center: i32) -> Vec<Vec<i32>> {
        let mut dis = HashMap::new();
        let mut dis_to_pointer: HashMap<i32, Vec<Vec<i32>>> = HashMap::new();
        for i in 0..rows {
            for j in 0..cols {
                let d = Self::distance(i, j, r_center, c_center);
                dis.insert(d, true);
                let pointer = vec![i, j];
                if let Some(dd) = dis_to_pointer.get_mut(&d) {
                    dd.push(pointer);
                } else {
                    dis_to_pointer.insert(d, vec![pointer]);
                }
            }
        };
        let mut result = Vec::new();
        let mut dis_vec = Vec::new();
        for (key, _) in dis.iter() {
            dis_vec.push(*key);
        }
        dis_vec.sort();
        dis_vec.iter().for_each(|d| {
            if let Some(pointers) = dis_to_pointer.get(d) {
                pointers.iter().for_each(|p| {
                    result.push(vec![p[0], p[1]]);
                })
            }
        });
        result
    }

    fn distance(r1: i32, c1: i32, r2: i32, c2: i32) -> i32 {
        (r1-r2).abs() + (c1-c2).abs()
    }
}