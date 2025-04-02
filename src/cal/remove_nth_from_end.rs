// https://leetcode.cn/problems/remove-nth-node-from-end-of-list/

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
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // 计算链表的长度
        let mut leng = 0;
        let mut current = &head;
        while let Some(node) = current {
          leng += 1;
          current = &node.next;
        }
        if leng == 0 || n > leng {
          return head
        }
        let nn = leng - n + 1;
        if nn == 1 {
          return head.unwrap().next
        }
        
        let mut dis = 1;
        let mut pre: &mut Box<ListNode> = head.as_mut().unwrap();
        loop {
          if dis == nn-1 {
            let next_node = pre.next.take();
            if let Some(node) = next_node {
              pre.next = node.next;
            }
            break;
          }
          pre = pre.next.as_mut().unwrap();
          dis += 1;
        }
        head
    }

    pub fn remove_nth_from_end_2(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
      // 创建虚拟头节点以简化删除逻辑
      let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
      let mut len = 0;
  
      // 第一次遍历计算链表长度
      let mut current = dummy.as_ref().unwrap().next.as_ref();
      while let Some(node) = current {
          len += 1;
          current = node.next.as_ref();
      }
  
      // 计算需要删除的节点的前驱位置
      let target = len - n;
      let mut current = dummy.as_mut().unwrap();
  
      // 移动到前驱节点
      for _ in 0..target {
          current = current.next.as_mut().unwrap();
      }
  
      // 删除目标节点
      let next_node = current.next.take();
      if let Some(node) = next_node {
          current.next = node.next;
      }
  
      // 返回处理后的链表头
      dummy.unwrap().next
  }
}