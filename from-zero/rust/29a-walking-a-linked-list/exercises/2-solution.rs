// Build a linked list from a slice using the dummy-head + tail-cursor pattern,
// then walk it and print each value.

struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}
impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

// Build a list of the given digits, in order.
fn from_slice(digits: &[i32]) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0)); // throwaway head
    let mut tail = &mut dummy; // &mut cursor at the current last node

    for &digit in digits {
        tail.next = Some(Box::new(ListNode::new(digit))); // attach after tail
        tail = tail.next.as_mut().unwrap(); // step the cursor onto the new node
    }

    dummy.next // the real list is everything after the dummy
}

fn main() {
    let list = from_slice(&[7, 0, 8]);

    // Walk and print each value with a borrowing cursor.
    let mut cursor = list.as_ref();
    while let Some(node) = cursor {
        print!("{} ", node.val); // 7 0 8
        cursor = node.next.as_ref();
    }
    println!();
}
