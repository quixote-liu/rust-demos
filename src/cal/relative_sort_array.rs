

// 给你两个数组，arr1 和 arr2，arr2 中的元素各不相同，arr2 中的每个元素都出现在 arr1 中。

// 对 arr1 中的元素进行排序，使 arr1 中项的相对顺序和 arr2 中的相对顺序相同。未在 arr2 中出现过的元素需要按照升序放在 arr1 的末尾。
// https://leetcode.cn/problems/relative-sort-array/

struct Solution{}

impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut start_index = 0;
        for i in 0..arr2.len() {
            let cur_sort = arr2[i];
            for j in start_index..arr1.len() {
                if arr1[j] == cur_sort {
                    
                }
            }    
        };

    }
}