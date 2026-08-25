// Call `.next()` on a REAL built-in iterator and watch it walk, then run dry.
//
// `"hi".bytes()` is a cursor over the two bytes of "hi": h is 104, i is 105.
// Each `.next()` hands back the next byte wrapped as `Some(...)`; once both are
// used up, it hands back `None` forever.
//
// `bytes` must be `mut`, because each `.next()` moves the cursor's position.

fn main() {
    let mut bytes = "hi".bytes();

    println!("{:?}", bytes.next());
    println!("{:?}", bytes.next());
    println!("{:?}", bytes.next());

    // Expected output (one per line):
    // Some(104)
    // Some(105)
    // None
}
