// A two-node list, then a three-node list, built by hand with Box.
//
// A Node holds a value and, maybe, a pointer to the next node on the heap.
// Fill in the type of `next` so the recursive struct actually compiles, then
// build 10 -> 20 -> 30 and print the three values by walking `.next`.

struct Node {
    val: i32,
    next: Option</* your code here: a Box pointing at the next Node */>,
}

fn main() {
    // Build 10 -> 20 -> 30. The last node's `next` is None (end of the list).
    // your code here

    // Walk the list and print each value: 10, 20, 30
    // your code here
}
