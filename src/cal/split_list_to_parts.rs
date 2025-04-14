// https://leetcode.cn/problems/split-linked-list-in-parts/description/

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution;

impl Solution {
    pub fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        if head.is_none() {
            return vec![]
        }

        let mut length = 0;
        let mut cur = &head;
        while let Some(node) = cur {
            length += 1;
            cur = &node.next;
        }

        let part_len = length / k;
        let r = length % k;
        let mut result: Vec<Option<Box<ListNode>>> = vec![];
        let mut current_head = head;

        for i in 0..k {
            let pl = part_len + if i < r {1} else {0};
            if pl == 0 {
                result.push(None);
                continue
            }
            // 开始对链表进行切分
            let mut cur = current_head.take();
            let mut tail = cur.as_mut();
            for _ in 1..pl {
                tail = tail.unwrap().next.as_mut();
            }
            current_head =  tail.unwrap().next.take();
            result.push(cur);
        }
        
        result
    }

    pub fn split_list_to_parts_v2(mut head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        // 计算链表长度
        let mut n = 0;
        let mut curr = &head;
        while let Some(node) = curr {
            n += 1;
            curr = &node.next;
        }

        let k = k as usize;
        let part_len = n / k;
        let r = n % k;

        let mut result = Vec::with_capacity(k);
        let mut current_head = head;

        for i in 0..k {
            let curr_len = part_len + if i < r { 1 } else { 0 };
            if curr_len == 0 {
                result.push(None);
                continue;
            }

            // 分割当前部分
            let mut head_part = current_head.take();
            let mut tail = &mut head_part;
            for _ in 0..curr_len - 1 {
                tail = &mut tail.as_mut().unwrap().next;
            }
            // 剩余部分作为新的current_head
            current_head = tail.as_mut().unwrap().next.take();
            result.push(head_part);
        }

        result
    }
}