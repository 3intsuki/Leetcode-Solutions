use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    fn dfs(root1: &Option<Rc<RefCell<TreeNode>>>, root2: &Option<Rc<RefCell<TreeNode>>>, level: i32) {
        match (root1, root2) {
            (Some(root1_ref), Some(root2_ref)) => {
                let mut root1_node = root1_ref.borrow_mut();
                let mut root2_node = root2_ref.borrow_mut();

                if level & 1 == 1 {
                    let temp = root1_node.val;
                    root1_node.val = root2_node.val;
                    root2_node.val = temp;
                }

                Self::dfs(&root1_node.left, &root2_node.right, level + 1);
                Self::dfs(&root1_node.right, &root2_node.left, level + 1);
            }
            _ => {}
        }
    }

    pub fn reverse_odd_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root_ref) = &root {
            let root_node = root_ref.borrow();
            Self::dfs(&root_node.left, &root_node.right, 1);
        }
        root
    }
}
