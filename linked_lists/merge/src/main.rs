// 21. Merge Two Sorted Lists: https://leetcode.com/problems/merge-two-sorted-lists
// It's a challenge so no theoretical solution or provided solution

use first::HealthyBrainSolution;
use fourth::SwapSolution;
use second::Solution;
use third::RecursiveSolution;

mod first;
mod fourth;
mod second;
mod third;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    fn push(&mut self, val: i32) {
        let new_node = ListNode::new(val);

        let out = std::mem::replace(self, new_node);
        self.next = Some(Box::new(out));
    }
}

fn main() {
    let mut list1 = ListNode::new(4);
    for i in [2, 1] {
        list1.push(i);
    }
    let mut list2 = ListNode::new(4);
    for i in [3, 1] {
        list2.push(i);
    }

    println!("1. {:?}\n2. {:?}", list1, list2);
    // println!(
    //     "R. {:?}",
    //     HealthyBrainSolution::merge_two_lists(Some(Box::new(list1)), Some(Box::new(list2)))
    // );
    // println!(
    //     "R. {:?}",
    //     Solution::merge_two_lists(Some(Box::new(list1)), Some(Box::new(list2)))
    // );
    // println!(
    //     "R. {:?}",
    //     RecursiveSolution::merge_two_lists(Some(Box::new(list1)), Some(Box::new(list2)))
    // );
    println!(
        "R. {:?}",
        SwapSolution::merge_two_lists(Some(Box::new(list1)), Some(Box::new(list2)))
    );
}
