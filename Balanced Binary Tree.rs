use std::{cell::RefCell, rc::Rc};

type Node = Rc<RefCell<TreeNode>>;

impl Solution {
    fn dfs(node: Option<&Node>) -> Option<i32> {
        if let Some(node) = node {
            let left_depth = Self::dfs(node.borrow().left.as_ref())?;
            let right_depth = Self::dfs(node.borrow().right.as_ref())?;
            if (right_depth - left_depth).abs() > 1 {
                return None;
            }
            Some(1 + left_depth.max(right_depth))
        } else {
            Some(0)
        }
    }

    pub fn is_balanced(root: Option<Node>) -> bool {
        Self::dfs(root.as_ref()).is_some()
    }
}
