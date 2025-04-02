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
        let mut parts_len: Vec<i32> = vec![0;k as usize];
        let mut index = 0;
        for _ in 0..leng {
            parts_len[index] += 1;
            index += 1;
            if index == parts_len.len() {
                index = 0;
            }
        }
        let mut res : Vec<Option<Box<ListNode>>> = vec![];
        let mut head = head;
        for part_len in parts_len {
            if part_len == 0 {
                res.push(None);
            }
            let mut cur = head.as_mut().unwrap();
            let mut len = part_len;
            while let Some(node) = cur.next.as_mut() {
                len -= 1;
                if len == 0 {
                    
                }
                
            }
            let mut tail = head.take();
            
        }
        


        // 长度大于链表长度时，返回单个node + Null
        if k >= leng {
            let mut index = k;
            let mut res: Vec<Option<Box<ListNode>>> = vec![];
            let mut next: Option<Box<ListNode>> = head;
            while let Some(mut node) = next.take() {
                next = node.next;
                node.next = None;
                res.push(Some(node));
                index -= 1;
            }
            for _ in 0..index {
                res.push(None);
            }
            return res;
        }
        // 长度小于链表长度



        vec![]
    }
}