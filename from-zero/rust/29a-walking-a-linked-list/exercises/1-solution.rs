// Walk a linked list with a BORROWING cursor (.as_ref()) and sum its values,
// leaving the original list intact.

struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}
impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

fn main() {
    // A hand-built list: 1 -> 2 -> 3
    let list = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode::new(3))),
        })),
    }));

    // Borrow into the list; the cursor is Option<&Box<ListNode>>.
    let mut total = 0;
    let mut cursor = list.as_ref();
    while let Some(node) = cursor {
        total += node.val; // read this node's value
        cursor = node.next.as_ref(); // hop to the next node (still borrowing)
    }
    println!("total: {total}"); // total: 6

    // The list was only ever borrowed, so it's still here and usable.
    println!("head still: {}", list.as_ref().unwrap().val); // head still: 1
}
