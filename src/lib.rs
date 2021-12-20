use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
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
            right: None,
        }
    }

    pub fn tree_node_wrap(node: TreeNode) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(node)))
    }

    pub fn new_left_right(val: i32, left: i32, right: i32) -> Self {
        TreeNode {
            val,
            left: Self::tree_node_wrap(Self::new(left)),
            right: Self::tree_node_wrap(Self::new(right)),
        }
    }

    pub fn new_left(val: i32, left: i32) -> Self {
        TreeNode {
            val,
            left: Self::tree_node_wrap(Self::new(left)),
            right: None,
        }
    }

    pub fn new_right(val: i32, right: i32) -> Self {
        let right = Self::new(right);
        TreeNode {
            val,
            left: None,
            right: Some(Rc::new(RefCell::new(right))),
        }
    }
}

pub struct Solution {}

impl Solution {
    /* left-Root-right */
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Solution::inorder_traversal_internal(&root)
    }

    /*
      - this is just to avoid the nasty ownership issue
        while traversing the tree
    */
    fn inorder_traversal_internal(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if let Some(refcell) = root {
            let node = refcell.borrow();
            let node_value = node.val;
            let mut left_vec = Solution::inorder_traversal_internal(&node.left);
            let mut right_vec = Solution::inorder_traversal_internal(&node.right);
            left_vec.push(node_value);
            left_vec.append(&mut right_vec);
            left_vec
        } else {
            vec![]
        }
    }

    pub fn test_fixture_1() -> Option<Rc<RefCell<TreeNode>>> {
        let r = TreeNode::new_left(2, 3);
        let mut root = TreeNode::new(1);
        root.right = TreeNode::tree_node_wrap(r);
        TreeNode::tree_node_wrap(root)
    }

    pub fn test_fixture_2() -> Option<Rc<RefCell<TreeNode>>> {
        None
    }

    pub fn test_fixture_3() -> Option<Rc<RefCell<TreeNode>>> {
        TreeNode::tree_node_wrap(TreeNode::new(1))
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_1() {
        let target = vec![1, 3, 2];
        let result = Solution::inorder_traversal(Solution::test_fixture_1());
        assert_eq!(result, target);
    }

    #[test]
    fn sample_2() {
        let target = vec![];
        let result = Solution::inorder_traversal(Solution::test_fixture_2());
        assert_eq!(result, target);
    }
    #[test]
    fn sample_3() {
        let target = vec![1];
        let result = Solution::inorder_traversal(Solution::test_fixture_3());
        assert_eq!(result, target);
    }
}
