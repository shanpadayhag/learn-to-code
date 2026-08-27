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

    // 1. Keep a cursor that borrows into the list: `let mut cursor = list.as_ref();`
    // 2. `while let Some(node) = cursor { ... }` — add node.val to a total,
    //    then hop: `cursor = node.next.as_ref();`
    // 3. Print the total (should be 6).
    // your code here

    // 4. Prove the list is untouched: print list's head value (1).
    // println!("head still: {}", list.as_ref().unwrap().val);
}
