// 1556. 千位分隔数
// 简单
// 相关标签
// 相关企业
// 提示
// 给你一个整数 n，请你每隔三位添加点（即 "." 符号）作为千位分隔符，并将结果以字符串格式返回。

 

// 示例 1：

// 输入：n = 987
// 输出："987"
// 示例 2：

// 输入：n = 1234
// 输出："1.234"
// 示例 3：

// 输入：n = 123456789
// 输出："123.456.789"
// 示例 4：

// 输入：n = 0
// 输出："0"
 

// 提示：

// 0 <= n < 2^31

struct Solution;

impl Solution {
    pub fn thousand_separator(n: i32) -> String {
        let mut nn = n.to_string();
        if nn.len() <= 3 {
            return nn;
        }
        let prefix_len = nn.len()%3;
        let mut index = prefix_len;
        if index == 0 {
            index = 3;
        }
        let mut insert_point: Vec<usize> = vec![index];
        loop {
            index += 3;
            if index >= nn.len() {
                break;
            }
            insert_point.push(index);
        }
        let mut res = String::new();
        for i in 0..insert_point.len() {
        }

        insert_point.iter().for_each(|i| {
            nn.insert(*i, '.');
        });
        nn
    }

    // pub fn thousand_separator(n: i32) -> String {
    //     let mut nn = n.to_string();
    //     if nn.len() <= 3 {
    //         return nn;
    //     }
    //     let prefix_len = nn.len()%3;
    //     let mut index = prefix_len;
    //     if index == 0 {
    //         index = 3;
    //     }
    //     let mut insert_point: Vec<usize> = vec![index];
    //     loop {
    //         index += 3;
    //         if index >= nn.len() {
    //             break;
    //         }
    //         insert_point.push(index+insert_point.len());
    //     }
    //     insert_point.iter().for_each(|i| {
    //         nn.insert(*i, '.');
    //     });
    //     nn
    // }
}

// let mut insert_index: Vec<i32> = Vec::new();
// let mut index = nn.len();
// loop {
//     index -= 3;
//     if index <= 0 {
//         break;
//     }
//     insert_index.push(index);   
// }



// loop {
//     index -= 2;
//     if index <= 0 {
//         break;
//     }
//     nn.insert(index, '.');
//     index -= 1;
// }