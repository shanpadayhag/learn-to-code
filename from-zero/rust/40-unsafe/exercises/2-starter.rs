// The point of `unsafe` is never to sprinkle it around. It is to write a few
// audited lines once, and hand out a SAFE function that nobody can misuse.
// That sandwich is how Vec, String, Rc and Mutex are all built.
//
// Run with:  rustc --edition 2024 2-starter.rs && ./2-starter

// 1. The dangerous core: sum the first `count` values with no bounds check.
//    Write the contract down first — an unwritten one is unkeepable.
//
// // SAFETY (the caller must guarantee):
// //   `count` is less than or equal to `values.len()`.
// unsafe fn sum_first_unchecked(values: &[u32], count: usize) -> u32 {
//     // loop 0..count, adding *values.get_unchecked(index)
// }

// 2. The safe wrapper. It checks the one thing the core cannot check, and
//    returns None instead of trusting the caller.
//
// fn sum_first(values: &[u32], count: usize) -> Option<u32> {
//     // bail out with None if count > values.len()
//     // otherwise call the core inside an unsafe block
// }

// 3. The same job with no unsafe anywhere, so you can compare answers.
//
// fn sum_first_safely(values: &[u32], count: usize) -> Option<u32> {
//     // hint: values[..count].iter().sum()
// }

fn main() {
    // 4. Run both over vec![4, 8, 15, 16, 23, 42] for count 0, 3 and 6, and
    //    assert_eq! that they agree. Then call sum_first with 99 and confirm
    //    you get None instead of a crash.

    // 5. Count how many `unsafe` keywords a CALLER of sum_first has to write.
    //    That number is the whole reason the wrapper exists.

    // 6. Now delete the bounds check from sum_first and call it with 99.
    //    Nothing warns. Nothing panics. You may get a number, a different
    //    number each run, or a segfault. sum_first is now UNSOUND: a safe
    //    function that lets safe code cause undefined behaviour.
    //
    //    Ask yourself where the bug is. It is not in the unsafe block — that
    //    block is unchanged and still correct. The mistake is in the SAFE
    //    code around it, which is why auditing "just the unsafe blocks" is
    //    never enough.

    // 7. Put the check back, then delete the `unsafe` keyword from the call
    //    to sum_first_unchecked and read error[E0133]. The keyword is not
    //    decoration: it makes you write down, at every call site, that you
    //    checked the contract by hand.
}
