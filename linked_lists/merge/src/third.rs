use crate::ListNode;

pub struct RecursiveSolution {}
impl RecursiveSolution {
    // Translated bonus solution
    // https://leetcode.com/problems/merge-two-sorted-lists/solutions/6048156/video-using-dummy-pointer-and-recursion-solution-as-a-bonus
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if list1 == None {
            return list2;
        }
        if list2 == None {
            return list1;
        }

        let mut list1 = list1.unwrap();
        let mut list2 = list2.unwrap();

        if list1.val > list2.val {
            std::mem::swap(&mut list1, &mut list2);
        }

        list1.next = Self::merge_two_lists(list1.next, Some(list2));
        return Some(list1);
    }
}
