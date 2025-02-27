// https://leetcode.cn/problems/average-salary-excluding-the-minimum-and-maximum-salary/
// 给你一个整数数组 salary ，数组里每个数都是 唯一 的，其中 salary[i] 是第 i 个员工的工资。

// 请你返回去掉最低工资和最高工资以后，剩下员工工资的平均值。

 

// 示例 1：

// 输入：salary = [4000,3000,1000,2000]
// 输出：2500.00000
// 解释：最低工资和最高工资分别是 1000 和 4000 。
// 去掉最低工资和最高工资以后的平均工资是 (2000+3000)/2= 2500
// 示例 2：

// 输入：salary = [1000,2000,3000]
// 输出：2000.00000
// 解释：最低工资和最高工资分别是 1000 和 3000 。
// 去掉最低工资和最高工资以后的平均工资是 (2000)/1= 2000
// 示例 3：

// 输入：salary = [6000,5000,4000,3000,2000,1000]
// 输出：3500.00000
// 示例 4：

// 输入：salary = [8000,9000,2000,3000,6000,1000]
// 输出：4750.00000
 

// 提示：

// 3 <= salary.length <= 100
// 10^3 <= salary[i] <= 10^6
// salary[i] 是唯一的。
// 与真实值误差在 10^-5 以内的结果都将视为正确答案。

struct Solution;

impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        if salary.len() < 2 {
            return 0 as f64;
        }
        let mut min_salary = salary[0];
        let mut max_salary = 0;
        let mut total: i32 = 0;
        salary.iter().for_each(|e| {
            if e.lt(&min_salary) {
                min_salary = *e;
            }
            if e.gt(&max_salary) {
                max_salary = *e;
            }
            total = total + *e;
        });

        (total-min_salary-max_salary) as f64 /(salary.len()-2) as f64
    }
}