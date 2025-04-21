// https://leetcode.cn/problems/count-complete-tree-nodes/

use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

struct Solution;

impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let n = node.borrow();
                1 + Self::count_nodes(n.left.clone()) + Self::count_nodes(n.right.clone())       
            }
        }
    }

    // pub fn count_nodes_v2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    //     if root.is_none() {
    //         return 0;
    //     }
        
    //     let root = root.unwrap();
    //     let left_height = {
    //         let mut height = 0;
    //         let mut node = root.clone();
    //         loop {
    //             height += 1;
    //             if node.borrow().left.is_none() {
    //                 break;
    //             }
    //             node = node.borrow().left.clone().unwrap();
    //         }
    //         height
    //     };
        
    //     let right_height = {
    //         let mut height = 0;
    //         let mut node = root.clone();
    //         loop {
    //             height += 1;
    //             if node.borrow().right.is_none() {
    //                 break;
    //             }
    //             node = node.borrow().right.clone().unwrap();
    //         }
    //         height
    //     };
        
    //     if left_height == right_height {
    //         // 满二叉树，节点数为2^h - 1
    //         (1 << left_height) - 1
    //     } else {
    //         1 + Self::count_nodes_v2(root.borrow().left.clone()) + Self::count_nodes_v2(root.borrow().right.clone())
    //     }
    // }
}