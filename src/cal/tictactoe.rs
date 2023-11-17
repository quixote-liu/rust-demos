// https://leetcode.cn/problems/find-winner-on-a-tic-tac-toe-game/

struct Solution{}

impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        // 开始复盘棋局
        let mut board: Vec<Vec<String>> = Vec::new();
        let mut node = "A".to_string();
        moves.iter().for_each(|e| {
            let x = e[0] as usize;
            let y = e[1] as usize;
            board[x][y] = node.clone();
            if node == "A" {
                node = "B".to_string();
            } else {
                node = "A".to_string();
            }
        });

        // 检查棋局状态
        let mut status = String::new();
        for i in 0..3 {
            // 检查横竖是否成立
            let mut column_ok = true;
            let first_node_c = board[i][0].clone();
            
            let mut row_ok = true;
            let first_node_r = board[0][i].clone();

            for j in 0..3 {
                let next_node_c = board[i][j].clone();
                if first_node_c == "" || next_node_c == "" {
                    status = "Pending".to_string();
                    column_ok = false;
                }
                if next_node_c != first_node_c {
                    column_ok = false;
                }

                let next_node_r = board[j][i].clone();
                if first_node_r == "" || next_node_r == "" {
                    status = "Pending".to_string();
                    row_ok = false;
                }
                if next_node_r != first_node_r {
                    row_ok = false;
                }
            }
            if column_ok {
                return first_node_c; // 返回游戏胜利者
            }
            if row_ok {
                return first_node_r; // 返回游戏胜利者
            }
        };

        // 检查左对角线是否成立
        let x_l = 0;
        let y_l = 0;
        for i in 0..3 {
            let first_node = board[0][0].clone();
        }
        // 检查右对角线是否成立

        if status == "" {"Draw".to_string()} else {status}
    }
}