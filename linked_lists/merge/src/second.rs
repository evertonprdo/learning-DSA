use crate::ListNode;

pub struct Solution {}
impl Solution {
    // Solution based on theory
    // https://leetcode.com/problems/merge-two-sorted-lists/solutions/6048156/video-using-dummy-pointer-and-recursion-solution-as-a-bonus
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut root = ListNode::new(0);
        let mut tail = &mut root.next;

        loop {
            match (list1.take(), list2.take()) {
                (Some(mut node1), Some(mut node2)) => {
                    if node1.val < node2.val {
                        list1 = node1.next.take();

                        *tail = Some(node1);
                        tail = Self::unwrap_next(tail);

                        list2 = Some(node2);
                    } else {
                        list2 = node2.next.take();

                        *tail = Some(node2);
                        tail = Self::unwrap_next(tail);

                        list1 = Some(node1);
                    }
                }
                (Some(mut node1), None) => {
                    list1 = node1.next.take();

                    *tail = Some(node1);
                    tail = Self::unwrap_next(tail);
                }
                (None, Some(mut node2)) => {
                    list2 = node2.next.take();

                    *tail = Some(node2);
                    tail = Self::unwrap_next(tail);
                }
                (None, None) => break,
            }
        }

        root.next.take()
    }
    fn unwrap_next(tail: &mut Option<Box<ListNode>>) -> &mut Option<Box<ListNode>> {
        match tail {
            Some(node) => &mut node.next,
            None => panic!(),
        }
    }
}
