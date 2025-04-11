use crate::ListNode;

pub struct SwapSolution {}
impl SwapSolution {
    // Ralph Tandetzky: Simple and Efficient Rust 8-liner
    // https://leetcode.com/problems/merge-two-sorted-lists/solutions/2947855/simple-and-efficient-rust-8-liner/
    pub fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut r = &mut l1;

        while l2.is_some() {
            if r.is_none() || l2.as_ref()?.val < r.as_ref()?.val {
                std::mem::swap(r, &mut l2);
            }
            r = &mut r.as_mut()?.next;
        }
        l1
    }
}
// "The code speaks for itself." (Ralph Tandetzky)
// Of course we all agree... 😅

// Loop Trace:
// ============
// At each iteration, `r` is a mutable reference walking through `l1`.
// If `l2`'s current node is smaller, it's spliced into `l1` via swap.
// Otherwise, `r` moves to the next node in `l1`.
//
// 1st loop ==================
//     l1 = [1, 2, 4]
//     r  = [1, 2, 4]
//     l2 = [1, 3, 4]
//
//     l2[0] < r[0] → false
//     → move r to r.next: [2, 4]
//
// 2nd loop ==================
//     l1 = [1, 2, 4]
//     r  =    [2, 4]
//     l2 = [1, 3, 4]
//
//     l2[0] < r[0] → true → swap(r, l2)
//     → r =  [1, 3, 4]
//     → l2 = [2, 4]
//     → move r to r.next: [3, 4]
//
// 3rd loop ==================
//     l1 = [1, 1, 3, 4]
//     r  =       [3, 4]
//     l2 = [2, 4]
//
//     l2[0] < r[0] → true → swap(r, l2)
//     → r =  [2, 4]
//     → l2 = [3, 4]
//     → move r to r.next: [4]
//
// 4th loop ==================
//     l1 = [1, 1, 2, 4]
//     r  =          [4]
//     l2 = [3, 4]
//
//     l2[0] < r[0] → true → swap(r, l2)
//     → r =  [3, 4]
//     → l2 = [4]
//     → move r to r.next: [4]
//
// 5th loop ==================
//     l1 = [1, 1, 2, 3, 4]
//     r  =             [4]
//     l2 = [4]
//
//     l2[0] < r[0] → false
//     → move r to r.next: [ ]
//
// 6th loop ==================
//     l1 = [1, 1, 2, 3, 4]
//     r  =                [ ]
//     l2 = [4]
//
//     r is None → true → swap(r, l2)
//     → r =  [4]
//     → l2 = [ ]
//     → move r to r.next: [ ]
//
// 7th loop ==================
//     l1 = [1, 1, 2, 3, 4, 4]
//     r  =                   [ ]
//     l2 = [ ]
//
//     l2 is None → exit loop
//     return l1: [1, 1, 2, 3, 4, 4]
