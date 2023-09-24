use std::ops::Sub;

struct Solution{}

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        for i in 0..(points.len()-1) {
            let cur = points[i].clone();
            let next = points[i+1].clone();
            let dis_y = (next[1] - cur[1]).abs();
            let dis_x = (next[0] - cur[0]).abs();
            if dis_x == 0 {
                count += dis_y;
            } else if dis_y == 0 {
                count += dis_x;
            } else {
                if dis_x == dis_y {
                    count += dis_x;
                } else {
                    count += dis_x.min(dis_y);
                    count += dis_x.sub(dis_y).abs();
                }
            };
        };
        return count;
    }
}