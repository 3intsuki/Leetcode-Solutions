impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut current = head;
        
        while let Some(mut node) = current {
            let next = node.next.take();
            node.next = result;
            result = Some(node);
            current = next;
        }
        
        result
    }
}
