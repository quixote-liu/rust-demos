// https://leetcode.cn/problems/find-winner-on-a-tic-tac-toe-game/

struct Solution{}

// [[0,2],[0,1],[2,1]] 输出为空
// [[2,1],[1,2],[1,0],[1,1],[0,2],[2,0],[0,1],[0,0]] 输出Draw 期望为Pending

impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        // 开始复盘棋局
        let mut board: Vec<Vec<String>> = Vec::new();
        let mut item = Vec::new();
        item.resize(3, "".to_string());
        board.resize(3, item);
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

        // 棋局状态
        let mut status = String::new();

        // 检查横是否成立
        for row in 0..3 {
            let first_node = board[row][0].clone();
            if first_node.as_str() == "" {
                status = "Pending".to_string();
                continue;
            }
            let mut row_ok = true;
            for col in 1..3 {
                let compared_node = board[row][col].clone();
                if compared_node.as_str() == "" {
                    status = "Pending".to_string();
                    row_ok = false;
                    break;
                }
                if first_node != compared_node {
                    row_ok = false;
                }
            }
            if row_ok {
                return first_node;
            }
        }

        // 检查竖是否成立
        for col in 0..3 {
            let first_node = board[0][col].clone();
            if first_node.as_str() == "" {
                status = "Pending".to_string();
                continue;
            }
            let mut col_ok = true;
            for row in 1..3 {
                let compared_node = board[row][col].clone();
                if compared_node.as_str() == "" {
                    status = "Pending".to_string();
                    col_ok = false;
                    break;
                }
                if first_node != compared_node {
                    col_ok = false;
                }
            }
            if col_ok {
                return first_node;
            }
        }

        // 检查对象线是否成立
        if board[0][0] != "" && board[0][0] == board[1][1] && board[0][0] == board[2][2] {
            return board[0][0].clone();
        }
        if board[2][0] != "" && board[2][0] == board[1][1] && board[2][0] == board[0][2] {
            return board[2][0].clone();
        }

        if status == "" {"Draw".to_string()} else {status}
    }
}