use std::{cell::RefCell, rc::Rc};

type Node = Rc<RefCell<TreeNode>>;

impl Solution {
    fn _helper(root: Option<&Node>, inverted: &mut Option<Node>) {
        if let Some(node) = root {
            inverted.replace(Rc::new(RefCell::new(TreeNode::new(node.borrow().val))));
            if let Some(inverted) = inverted.as_ref() {
                Self::_helper(node.borrow().right.as_ref(), &mut inverted.borrow_mut().left);
                Self::_helper(node.borrow().left.as_ref(), &mut inverted.borrow_mut().right);
            }
        }
    }

    pub fn invert_tree(root: Option<Node>) -> Option<Node> {
        let mut inverted = None;
        Self::_helper(root.as_ref(), &mut inverted);
        inverted
    }
}
