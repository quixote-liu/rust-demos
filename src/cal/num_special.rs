struct Solution;

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        for i in 0..mat.len() {
            for j in 0..mat[i].len() {
                if mat[i][j] == 1 {
                    // 行中的其他元素是否为零
                    let mut ok1 = true;
                    for n in 0..mat[i].len() {
                        if n == j {
                            continue
                        }
                        if mat[i][n] != 0 {
                            ok1 = false;
                            break;
                        }
                    }

                    if ok1 {
                        // 列中的其他元素是否为零
                        let mut ok2 = true;
                        for m in 0..mat.len() {
                            if m == i {
                                continue
                            }
                            if mat[m][j] != 0 {
                                ok2 = false;
                            }
                        }
                        if ok2 {
                            res += 1;
                        }
                    }
                }
            }
        }
        res
    }
}