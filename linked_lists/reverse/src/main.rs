// 206. Reverse Linked List: https://leetcode.com/problems/reverse-linked-list

use std::mem;

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
}

fn main() {
    let mut head = ListNode::new(1);
    push(&mut head, 5);
    push(&mut head, 4);
    push(&mut head, 3);
    push(&mut head, 2);

    println!("{:?}", head);
    println!("{:?}", reverse_list2(Some(Box::new(head))));
}

// This is me trying to solve the problem on my own
fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head?;
    let mut new_list = ListNode::new(0);

    while let Some(node) = head.next.take() {
        push(&mut new_list, head.val);
        head = node;
    }

    push(&mut new_list, head.val);
    new_list.next
}
fn push(list: &mut ListNode, val: i32) {
    let mut new_node = Box::new(ListNode::new(val));

    match list.next.take() {
        Some(node) => {
            new_node.next = Some(node);
            list.next = Some(new_node);
        }
        None => list.next = Some(new_node),
    }
}

// This is me trying to translate the solution
fn reverse_list2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head?;
    let mut list = Box::new(ListNode::new(0));

    // Why so many extra steps? Well, it made sense when I was doing it.
    // In short, I was overthinking the borrow checker.
    loop {
        match head.next.take() {
            Some(node) => {
                let out = mem::replace(&mut head, node);
                prepend(&mut list, out);
            }
            None => {
                prepend(&mut list, head);
                break;
            }
        }
    }

    list.next
}
fn prepend(node: &mut Box<ListNode>, mut new: Box<ListNode>) {
    match node.next.take() {
        Some(n) => {
            new.next = Some(n);
            node.next = Some(new);
        }
        None => node.next = Some(new),
    };
}

// I couldn't make an implementation based only on theory
// I couldn't translate the provided solution

// But Someone got it
fn reverse_list3(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;

    while let Some(mut current) = head {
        // For some reason Rust made me believe that `head` was unusable after move
        // but 'mut' means it's still in scope — just invalid until you assign something new
        // so of course you can still give it another value, why would that be forbidden?
        head = current.next.take();

        current.next = prev;
        prev = Some(current);
    }

    prev
}
