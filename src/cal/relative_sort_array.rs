

// 给你两个数组，arr1 和 arr2，arr2 中的元素各不相同，arr2 中的每个元素都出现在 arr1 中。

// 对 arr1 中的元素进行排序，使 arr1 中项的相对顺序和 arr2 中的相对顺序相同。未在 arr2 中出现过的元素需要按照升序放在 arr1 的末尾。
// https://leetcode.cn/problems/relative-sort-array/

use core::time;

struct Solution{}

impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut arr1_map = std::collections::HashMap::new();
        arr1.iter().for_each(|element| {
            arr1_map.entry(element).and_modify(|val| *val += 1).or_insert(1);
        });
        let mut new_arr = Vec::new();
        arr2.iter().for_each(|element| {
            if let Some(&times) = arr1_map.get(element) {
                for i in 0..times {
                    new_arr.push(element.clone());
                };
                arr1_map.remove(element);
            }
        });
        let mut sort_keys = Vec::new();
        for (key, val) in arr1_map.clone() {
            sort_keys.push(key.clone());
        };
        sort_keys.sort();
        new_arr.append(&mut sort_keys);

        // for key in sort_keys {
        //     if let Some(&times) = arr1_map.get(key) {
        //         for i in 0..times {
        //             new_arr.push(key.clone());
        //         };
        //     };
        // };
        return new_arr;
    }
}