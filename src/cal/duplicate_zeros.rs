// 给你一个长度固定的整数数组 arr ，请你将该数组中出现的每个零都复写一遍，并将其余的元素向右平移。

// 注意：请不要在超过该数组长度的位置写入元素。请对输入的数组 就地 进行上述修改，不要从函数返回任何东西。

// leetcode链接： https://leetcode.cn/problems/duplicate-zeros/
// 通过连接：https://leetcode.cn/problems/duplicate-zeros/submissions/

// 例如：
// 输入：arr = [1,0,2,3,0,4,5,0]
// 输出：[1,0,0,2,3,0,0,4]
// 解释：调用函数后，输入的数组将被修改为：[1,0,0,2,3,0,0,4]

struct Solution{}

impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let arr_len = arr.len();
        let mut index = 0;
        while index < arr_len {
            if arr[index] == 0 {
                arr.insert(index+1, 0);
                arr.truncate(arr_len);
                index = index + 2;
            } else {
                index += 1
            }
        }
    }
    // 不使用自带的vec函数情况下，可以使用双指针思路
}