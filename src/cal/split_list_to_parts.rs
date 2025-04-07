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
    pub fn split_list_to_parts(mut head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        if head.is_none() {
            return vec![]
        }

        let mut leng = 0;
        let mut cur = &head;
        while let Some(node) = cur {
            leng += 1;
            cur = &node.next;
        }
        let mut part_lens: Vec<i32> = vec![0;k as usize];
        let mut index = 0;
        for _ in 0..leng {
            part_lens[index] += 1;
            index += 1;
            if index == part_lens.len() {
                index = 0;
            }
        }
        let mut res : Vec<Option<Box<ListNode>>> = vec![];
        let mut head = head;
        for part_len in part_lens {
            if part_len == 0 {
                res.push(None);
                continue;
            }
            let mut cur = head.as_mut().unwrap();
            let mut len = part_len;
            while let Some(node) = cur.next.as_mut() {
                len -= 1;
                cur = node;
                if len == 0 {
                    res.push(head);
                    head = cur.next.take();
                    break;
                }
            }

        }
        vec![]
    }
}