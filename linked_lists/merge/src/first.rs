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
                        list1 = node1.next.take();
                        Self::transversal_push(&mut root, node1);

                        list2 = Some(node2);
                    } else {
                        list2 = node2.next.take();
                        Self::transversal_push(&mut root, node2);

                        list1 = Some(node1);
                    }
                }
                (Some(mut node1), None) => {
                    list1 = node1.next.take();
                    Self::transversal_push(&mut root, node1);
                }
                (None, Some(mut node2)) => {
                    list2 = node2.next.take();
                    Self::transversal_push(&mut root, node2);
                }
                (None, None) => break,
            }
        }

        root.next.take()
    }
    fn transversal_push(root: &mut ListNode, node: Box<ListNode>) {
        match &mut root.next {
            Some(next) => {
                Self::transversal_push(next, node);
            }
            None => {
                root.next = Some(node);
            }
        }
    }
}
