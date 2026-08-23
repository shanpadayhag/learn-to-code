// This is the exact situation from the Two Sum solution: you have references
// (&i32) but you need to put OWNED i32 values into a Vec<i32>. The `*` is what
// turns a &i32 into the i32 it points to. Without it, `vec![ra, rb]` would be a
// Vec<&i32>, and the type annotation below would refuse to compile.

fn main() {
    let a = 5;
    let b = 8;
    let ra = &a;
    let rb = &b;

    // Build a Vec<i32> from the two references by dereferencing each with `*`.
    // let pair: Vec<i32> = vec![ ... , ... ];
    // your code here

    // Print it with {:?} (Debug formatting for the whole vector).
    // your code here

    // Expected:
    //   [5, 8]
}
