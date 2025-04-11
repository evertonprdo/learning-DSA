use crate::ListNode;

pub struct HealthyBrainSolution {}
impl HealthyBrainSolution {
    // My Solution: Consume both lists while yields a new list having a really bad Big O
    // and still getting a 0ms solution because It's Rust

    // Time Complexity: O(n^2)
    // Space Complexity: O(n)
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut root = ListNode::new(0);

        loop {
            match (list1.take(), list2.take()) {
                (Some(mut node1), Some(mut node2)) => {
                    if node1.val < node2.val {
                        Self::transversal_push(&mut root, node1.val);
                        list1 = node1.next.take();
                        list2 = Some(node2);
                    } else {
                        Self::transversal_push(&mut root, node2.val);
                        list2 = node2.next.take();
                        list1 = Some(node1);
                    }
                }
                (Some(mut node1), None) => {
                    Self::transversal_push(&mut root, node1.val);
                    list1 = node1.next.take();
                }
                (None, Some(mut node2)) => {
                    Self::transversal_push(&mut root, node2.val);
                    list2 = node2.next.take();
                }
                (None, None) => break,
            }
        }

        root.next.take()
    }
    fn transversal_push(root: &mut ListNode, val: i32) {
        match &mut root.next {
            Some(node) => {
                Self::transversal_push(node, val);
            }
            None => {
                root.next = Some(Box::new(ListNode::new(val)));
            }
        }
    }
}
