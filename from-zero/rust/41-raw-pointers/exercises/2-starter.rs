// split_at_mut: two &mut into one slice at once. The borrow checker MUST
// reject it — it sees one slice being borrowed mutably twice — and yet it is
// obviously fine, because the two halves never overlap.
//
// That gap is exactly what raw pointers are for: you can see the proof, the
// compiler cannot, so you write it down with an assert and take the blame.
//
// Run with:  rustc --edition 2024 2-starter.rs && ./2-starter

use std::slice;

// 1. FIRST, try it the honest way and read the error. Uncomment this:
//
// fn split_at_mut(values: &mut [i32], middle: usize) -> (&mut [i32], &mut [i32]) {
//     (&mut values[..middle], &mut values[middle..])
// }
//
//    error[E0499]: cannot borrow `*values` as mutable more than once at a time
//
//    The compiler is not being fussy. Telling that `..middle` and `middle..`
//    are disjoint needs reasoning about the VALUE of `middle`, which is not
//    something a type system does.

// 2. Now the raw-pointer version.
//
// fn split_at_mut(values: &mut [i32], middle: usize) -> (&mut [i32], &mut [i32]) {
//     let length = values.len();
//     let start = values.as_mut_ptr();      // take the address BEFORE either half exists
//     assert!(middle <= length);            // this line is the safety argument
//     unsafe {
//         (
//             slice::from_raw_parts_mut(start, middle),
//             slice::from_raw_parts_mut(start.add(middle), length - middle),
//         )
//     }
// }
//
//    from_raw_parts_mut(pointer, count) builds a &mut [i32] out of an address
//    and a length. It is an unsafe fn because it invents a slice — and a
//    lifetime — out of a number you handed it.

fn main() {
    // 3. Split [1, 2, 3, 4, 5, 6] at 3, multiply the left half by 10 and add
    //    100 to the right half — both halves live at the same time — then
    //    print the original array and assert_eq! the result.

    // 4. Check the two edges: split at 0 and split at len(). Both should give
    //    you one empty slice and one full one, with no panic.

    // 5. Delete the assert! and call it with middle = 99. In a DEBUG build a
    //    different check catches it first ("attempt to subtract with
    //    overflow") — that is luck, not safety. Overflow checks are off in
    //    release. Work out what `length - middle` becomes there, and how big
    //    the slice you just handed out would be.
    //
    //    Then notice the important part: split_at_mut is a SAFE function. Its
    //    callers write no keyword. Deleting one line made safe code able to
    //    cause undefined behaviour — that is what "unsound" means.

    // 6. Put the assert back, then replace `start.add(middle)` with `start`.
    //    Nothing complains, and you now hold two &mut to the same i32 — the
    //    exact thing the borrow checker exists to prevent.
    let _ = slice::from_ref(&0i32);
}
