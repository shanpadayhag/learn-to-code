// Count how many letters two words share at the START — their common prefix
// length. This is the exact scan from the longest-common-prefix code.
//
// Build the chain on the two byte-cursors:
//   first.bytes()               -> h e l l o ...
//   .zip(second.bytes())        -> pair them up: (f,f) (l,l) (o,o) ...  stops when either ends
//   .take_while(|(a, b)| a == b)-> keep pairs while the two bytes match; STOP at the first mismatch
//   .count()                    -> the consumer: run it and return how many pairs survived
//
// For "flower" and "flight": (f,f)✓ (l,l)✓ (o,i)✗ stop  ->  2

fn main() {
    let first = "flower";
    let second = "flight";

    let shared_prefix_length = first
        .bytes()
        .zip(second.bytes())
        .take_while(|(a, b)| a == b)
        .count();

    println!("{shared_prefix_length}");
    // Expected output: 2
}
