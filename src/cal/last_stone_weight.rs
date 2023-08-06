


struct Solution {}

impl Solution {
    pub fn last_stone_weight(mut stones: Vec<i32>) -> i32 {
        let (x, y, leng) = max_and_leng_of_stones(&stones);
        if leng == 0 {
            return 0;
        } else if leng == 1 {
            return y.value;
        }
        if x.value == y.value {
            stones[x.index as usize] = -1;
            stones[y.index as usize] = -1;
        } else {
            stones[x.index as usize] = -1;
            stones[y.index as usize] = y.value - x.value;
        }
        return Self::last_stone_weight(stones);
    }
}

struct Stone {
    index: i32,
    value: i32,
}

impl Stone {
    fn new(index: i32, value: i32) -> Stone {
        return Stone { index: index, value: value};
    }
}

fn max_and_leng_of_stones(stones: &Vec<i32>) -> (Stone,Stone, i32) {
    let mut x = Stone::new(0, 0);
    let mut y = Stone::new(0, 0);
    let mut leng = 0;
    for i in 0..stones.len() {
        let val = stones[i];
        if val == -1 {
            continue;
        }
        leng += 1;
        if val > y.value {
            x = y;
            y = Stone::new(i.clone() as i32, val.clone());
        } else if val > x.value {
            x = Stone::new(i.clone() as i32, val.clone());
        }
    };
    return (x, y, leng);
}