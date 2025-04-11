// 876. Middle of the Linked List: https://leetcode.com/problems/middle-of-the-linked-list

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
impl Drop for ListNode {
    fn drop(&mut self) {
        println!("dropped: {:?}", self)
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

    println!("{:?}", Solution::middle_node3(Some(Box::new(head))));
}

struct Solution {}
impl Solution {
    // Based on Theory
    // Simple approach: count the nodes, then return the upper middle node
    pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut length = 0;
        {
            let mut current = head.as_ref();
            while let Some(node) = current {
                length += 1;
                current = node.next.as_ref();
            }
        }

        let mid = length / 2;

        for _ in 0..mid {
            if let Some(mut node) = head.take() {
                head = node.next.take();
            }
        }

        head
    }

    // Slow and Fast approach: My translated version of the provided solution
    pub fn middle_node2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = head.as_ref();
        let mut fast = head.as_ref();

        loop {
            // After this, I saw a lot of versions of this solution and feel better with this "ugly" code
            // because there are some really much uglier versions of this section...
            fast.map(|node| fast = node.next.as_ref());
            let next = fast.map(|node| fast = node.next.as_ref());

            if next == None {
                break;
            }

            slow.map(|node| slow = node.next.as_ref());
        }

        slow.cloned()
    }

    // Slow and Fast approach: Someone's safe version
    pub fn middle_node5(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = &head;
        let mut fast = &head;

        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }

        slow.clone()
    }

    // fifty_four_k's "fast and slow" translated version using unsafe code
    // Drop says: missing some nodes
    // Miri says: memory leaked
    // https://leetcode.com/problems/middle-of-the-linked-list/solutions/2878748/rust-2-pointers-safe-and-unsafe-with-no-cloning/
    pub fn middle_node3(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        unsafe {
            let mut s = Box::into_raw(head?);
            let mut f = s.as_ref();

            while f.is_some() && f.unwrap().next.is_some() {
                f = f?.next.as_ref()?.next.as_deref();
                s = Box::into_raw((*s).next.take()?);
            }

            Some(Box::from_raw(s))
        }
    }

    // GPT's Reasoning-mode unsafe version
    // Drop says: "I see all nodes"
    // Miri says: Undefined Behavior
    // I tried to give the Miri alert to GPT, but they took 5 minutes to give me a solution that doesn't even compile
    pub fn middle_node4(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow: *mut Option<Box<ListNode>> = &mut head;
        let mut fast: *mut Option<Box<ListNode>> = &mut head;

        unsafe {
            while (*fast).is_some() && (*fast).as_ref().unwrap().next.is_some() {
                slow = &mut (*slow).as_mut().unwrap().next as *mut _;
                fast = &mut (*fast).as_mut().unwrap().next.as_mut().unwrap().next as *mut _;
            }

            std::mem::take(&mut *slow)
        }
    }
}
