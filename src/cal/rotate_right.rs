// https://leetcode.cn/problems/rotate-list/description/

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
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if let Some(mut head) = head {
            let mut length = 1;
            let mut cur: &mut Box<ListNode>= &mut head;
            while let Some(node) = cur.next.as_mut() {
                cur = node;
                length += 1;
            }

            for _ in 0..(k % length) {
                let mut index = length;
                let mut cur: &mut Box<ListNode> = &mut head;
                loop {
                    if index == 2 {
                        let mut tail = cur.next.take().unwrap();
                        // 尾部作为头部, 头部作为第二个元素
                        tail.next = Some(head);
                        head = tail;
                        break;
                    }
                    index -= 1;
                    if let Some(node) = cur.next.as_mut() {
                        cur = node;
                    } else {
                        break;
                    }
                }
            }
            return Some(head);
        }
        head
    }
}