use std::io::repeat;

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
        let new_node = Box::new(ListNode::new(val));

        let out = std::mem::replace(self, *new_node);
        self.next = Some(Box::new(out));
    }
}

fn main() {
    // Less verbose, but feels more confusing (to me).
    // let mut root = ListNode::new(5);
    // for i in (1..=4).rev() {
    //    root.push(i);
    // }

    // More verbose, but easier to read and visualize what's going on
    let mut i = 5;
    let mut head = ListNode::new(i);

    while i > 1 {
        i -= 1;
        head.push(i);
    }

    println!("{:?}\n", head);
    println!("{:?}", Solution::middle_node(Some(Box::new(head))));
}
struct Solution {}
impl Solution {
    // Simple approach: count the nodes, then return the upper middle node
    pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut p = head.as_deref();
        let mut size = 0;

        while p != None {
            p.map(|node| {
                size += 1;
                p = node.next.as_deref()
            });
        }

        let mid = if size & 1 == 1 {
            size / 2 + 1 // Odd
        } else {
            size / 2 // Pair
        };

        while size > mid {
            if let Some(node) = head.take() {
                head = node.next;
                size -= 1;
            }
        }

        head
    }
}
