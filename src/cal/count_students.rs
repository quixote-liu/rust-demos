// https://leetcode.cn/problems/number-of-students-unable-to-eat-lunch/description/
// 1700. 无法吃午餐的学生数量
// 已解答
// 简单
// 相关标签
// 相关企业
// 提示
// 学校的自助午餐提供圆形和方形的三明治，分别用数字 0 和 1 表示。所有学生站在一个队列里，每个学生要么喜欢圆形的要么喜欢方形的。
// 餐厅里三明治的数量与学生的数量相同。所有三明治都放在一个 栈 里，每一轮：

// 如果队列最前面的学生 喜欢 栈顶的三明治，那么会 拿走它 并离开队列。
// 否则，这名学生会 放弃这个三明治 并回到队列的尾部。
// 这个过程会一直持续到队列里所有学生都不喜欢栈顶的三明治为止。

// 给你两个整数数组 students 和 sandwiches ，其中 sandwiches[i] 是栈里面第 i​​​​​​ 个三明治的类型（i = 0 是栈的顶部）， students[j] 是初始队列里第 j​​​​​​ 名学生对三明治的喜好（j = 0 是队列的最开始位置）。请你返回无法吃午餐的学生数量。

 

// 示例 1：

// 输入：students = [1,1,0,0], sandwiches = [0,1,0,1]
// 输出：0 
// 解释：
// - 最前面的学生放弃最顶上的三明治，并回到队列的末尾，学生队列变为 students = [1,0,0,1]。
// - 最前面的学生放弃最顶上的三明治，并回到队列的末尾，学生队列变为 students = [0,0,1,1]。
// - 最前面的学生拿走最顶上的三明治，剩余学生队列为 students = [0,1,1]，三明治栈为 sandwiches = [1,0,1]。
// - 最前面的学生放弃最顶上的三明治，并回到队列的末尾，学生队列变为 students = [1,1,0]。
// - 最前面的学生拿走最顶上的三明治，剩余学生队列为 students = [1,0]，三明治栈为 sandwiches = [0,1]。
// - 最前面的学生放弃最顶上的三明治，并回到队列的末尾，学生队列变为 students = [0,1]。
// - 最前面的学生拿走最顶上的三明治，剩余学生队列为 students = [1]，三明治栈为 sandwiches = [1]。
// - 最前面的学生拿走最顶上的三明治，剩余学生队列为 students = []，三明治栈为 sandwiches = []。
// 所以所有学生都有三明治吃。
// 示例 2：

// 输入：students = [1,1,1,0,0,1], sandwiches = [1,0,0,0,1,1]
// 输出：3
 

// 提示：

// 1 <= students.length, sandwiches.length <= 100
// students.length == sandwiches.length
// sandwiches[i] 要么是 0 ，要么是 1 。
// students[i] 要么是 0 ，要么是 1 。

struct Solution;

impl Solution {
    pub fn count_students(mut students: Vec<i32>, mut sandwiches: Vec<i32>) -> i32 {
        let mut count = 0;
        loop {
            if count == students.len() || students.len() == 0 || sandwiches.len() == 0 {
                return students.len() as i32;
            }
            if students[0] == sandwiches[0] {
                sandwiches.remove(0);
                students.remove(0);
                count = 0;
            } else {
                let first = students[0];
                students.remove(0);
                students.push(first);
                count += 1;
            }
        }
    }
}