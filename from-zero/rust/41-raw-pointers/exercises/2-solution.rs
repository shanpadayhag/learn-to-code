// split_at_mut: two &mut into one slice at once. The borrow checker MUST
// reject it — it sees one slice being borrowed mutably twice — and yet it is
// obviously fine, because the two halves never overlap.
//
// That gap is exactly what raw pointers are for: you can see the proof, the
// compiler cannot, so you write it down with an assert and take the blame.
//
// Run with:  rustc --edition 2024 2-solution.rs && ./2-solution

use std::slice;

fn split_at_mut(values: &mut [i32], middle: usize) -> (&mut [i32], &mut [i32]) {
    let length = values.len();

    // Take the address BEFORE either half exists. From here on the two
    // halves are built from one pointer, not from each other.
    let start = values.as_mut_ptr();

    // This assert is the entire safety argument. Without it `middle` could
    // exceed `length`, the second half would run past the end of the buffer,
    // and this safe function would become unsound.
    assert!(middle <= length);

    unsafe {
        (
            slice::from_raw_parts_mut(start, middle),
            slice::from_raw_parts_mut(start.add(middle), length - middle),
        )
    }
}

fn main() {
    let mut readings = [1, 2, 3, 4, 5, 6];
    println!("before: {readings:?}");

    let (left, right) = split_at_mut(&mut readings, 3);

    // Two live &mut into one array. Every line below would be a borrow error
    // if the halves had come from anywhere but raw pointers.
    for value in left.iter_mut() {
        *value *= 10;
    }
    for value in right.iter_mut() {
        *value += 100;
    }

    println!("left  {left:?}");
    println!("right {right:?}");
    println!("after:  {readings:?}");
    assert_eq!(readings, [10, 20, 30, 104, 105, 106]);

    let mut empty_side = [7, 8];
    let (nothing, everything) = split_at_mut(&mut empty_side, 0);
    println!();
    println!("split at 0: {nothing:?} and {everything:?}");
    assert!(nothing.is_empty());

    let (all, none) = split_at_mut(&mut empty_side, 2);
    println!("split at len: {all:?} and {none:?}");
    assert!(none.is_empty());

    println!();
    println!("the standard library's version does exactly this: {:?}",
             [1, 2, 3, 4].split_at_mut(2));
}

// before: [1, 2, 3, 4, 5, 6]
// left  [10, 20, 30]
// right [104, 105, 106]
// after:  [10, 20, 30, 104, 105, 106]
//
// split at 0: [] and [7, 8]
// split at len: [7, 8] and []
//
// the standard library's version does exactly this: ([1, 2], [3, 4])
//
// Three things to try.
//
// 1. Write it without raw pointers:
//
//        (&mut values[..middle], &mut values[middle..])
//
//    error[E0499]: cannot borrow `*values` as mutable more than once at a time
//
//    The compiler is not being fussy. It genuinely cannot tell that `..middle`
//    and `middle..` are disjoint — that would need it to reason about the
//    VALUE of `middle`, which is not something a type system does.
//
// 2. Delete the assert! and call split_at_mut(&mut readings, 99). The first
//    half claims 99 elements of a 6-element buffer, and the second asks for
//    `6 - 99`. In a debug build a DIFFERENT check happens to catch it first:
//
//    thread 'main' panicked at 2-solution.rs:
//    attempt to subtract with overflow
//
//    That panic is luck, not safety — overflow checks are off in release
//    builds, where the subtraction wraps to about 18 quintillion and you get
//    a slice pointing at memory you do not own. Note what just happened:
//    split_at_mut is a SAFE function, its callers write no keyword, and
//    deleting one line made it able to cause undefined behaviour. That is
//    what "unsound" means, and the assert was the whole thing preventing it.
//
// 3. Replace `start.add(middle)` with `start` and watch both halves overlap.
//    Nothing complains, and you have handed out two &mut to the same i32 —
//    the exact thing the borrow checker exists to prevent.
