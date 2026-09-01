// The point of `unsafe` is never to sprinkle it around. It is to write a few
// audited lines once, and hand out a SAFE function that nobody can misuse.
// That sandwich is how Vec, String, Rc and Mutex are all built.
//
// Run with:  rustc --edition 2024 2-solution.rs && ./2-solution

// The dangerous core. `unsafe` on the signature means: calling me is a
// promise. The promise is written down, because an unwritten contract is
// one nobody can keep.
//
// SAFETY (the caller must guarantee):
//   `count` is less than or equal to `values.len()`.
unsafe fn sum_first_unchecked(values: &[u32], count: usize) -> u32 {
    let mut total = 0;
    for index in 0..count {
        total += unsafe { *values.get_unchecked(index) };
    }
    total
}

// The safe wrapper. It checks the one thing the core cannot check, so the
// promise is kept by construction and no caller can ever break it.
fn sum_first(values: &[u32], count: usize) -> Option<u32> {
    if count > values.len() {
        return None;
    }
    Some(unsafe { sum_first_unchecked(values, count) })
}

// The same job with no unsafe at all, for comparison.
fn sum_first_safely(values: &[u32], count: usize) -> Option<u32> {
    if count > values.len() {
        return None;
    }
    Some(values[..count].iter().sum())
}

fn main() {
    let readings = vec![4, 8, 15, 16, 23, 42];

    for count in [0, 3, 6] {
        let wrapped = sum_first(&readings, count);
        let plain = sum_first_safely(&readings, count);
        println!("first {count}: unsafe core {wrapped:?}, all-safe {plain:?}");
        assert_eq!(wrapped, plain);
    }

    let too_many = sum_first(&readings, 99);
    println!("first 99: {too_many:?}");
    assert_eq!(too_many, None);

    println!("callers that had to write `unsafe`: 0");
}

// first 0: unsafe core Some(0), all-safe Some(0)
// first 3: unsafe core Some(27), all-safe Some(27)
// first 6: unsafe core Some(108), all-safe Some(108)
// first 99: None
// callers that had to write `unsafe`: 0
//
// Two things to try, in this order.
//
// 1. Delete the `if count > values.len()` guard from sum_first, then call
//    sum_first(&readings, 99). Nothing warns. Nothing panics. You may get a
//    number, or a different number each run, or a segfault. sum_first is now
//    UNSOUND: it is a safe function that lets safe code cause undefined
//    behaviour. That is the bug the wrapper exists to prevent, and no amount
//    of unsafe blocks would have caught it — the mistake is in the SAFE code.
//
// 2. Put the guard back and delete the `unsafe` keyword from the call:
//
//    error[E0133]: call to unsafe function `sum_first_unchecked` is unsafe
//                  and requires unsafe function or block
//
//    The keyword is not decoration. It is the compiler making you write down,
//    at every single call site, that you have checked the contract by hand.
