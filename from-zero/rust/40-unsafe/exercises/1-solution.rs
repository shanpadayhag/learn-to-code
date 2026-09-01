// `unsafe` does NOT switch off the borrow checker. It unlocks exactly five
// extra abilities and changes nothing else. This file uses three of them and
// proves the fourth thing: the ordinary rules are all still running.
//
// Run with:  rustc --edition 2024 1-solution.rs && ./1-solution

static mut LAUNCH_COUNT: u32 = 0;

// Superpower 2: an unsafe fn. The `unsafe` on the SIGNATURE restricts the
// caller; it does not make the body permissive. Since edition 2024 the body
// still needs its own unsafe block, which is why there are two here.
unsafe fn record_launch() {
    unsafe {
        LAUNCH_COUNT += 1;
    }
}

fn launches_so_far() -> u32 {
    // Superpower 3: reading a `static mut`. `&raw const` takes the address
    // without ever creating a reference — a plain `println!("{LAUNCH_COUNT}")`
    // is a hard error in edition 2024 (see the note at the bottom).
    unsafe { *(&raw const LAUNCH_COUNT) }
}

fn main() {
    unsafe {
        record_launch();
        record_launch();
        record_launch();
    }
    println!("launches: {}", launches_so_far());

    let readings = vec![12, 47, 3, 91];

    let checked = readings[2];
    // Superpower 1 in disguise: get_unchecked is an unsafe fn that skips the
    // bounds check. Same answer, one fewer comparison — and the promise that
    // the index is in range is now yours, not the compiler's.
    let unchecked = unsafe { *readings.get_unchecked(2) };
    println!("readings[2] checked {checked}, unchecked {unchecked}");

    let mut total = 0;
    for index in 0..readings.len() {
        total += unsafe { *readings.get_unchecked(index) };
    }
    println!("total {total}");

    let owner = 5;
    let borrowed = &owner;
    println!("the borrow checker is still on: {owner} == {borrowed}");
}

// launches: 3
// readings[2] checked 3, unchecked 3
// total 153
// the borrow checker is still on: 5 == 5
//
// Now try to break a rule INSIDE an unsafe block:
//
//     let mut owner = 5;
//     let borrowed = &owner;
//     unsafe {
//         owner += 1;
//         println!("{borrowed}");
//     }
//
// error[E0506]: cannot assign to `owner` because it is borrowed
// warning: unnecessary `unsafe` block
//
// Read those two together. The borrow error still fires, and the compiler
// even tells you the unsafe block bought you nothing — because none of the
// five superpowers were used inside it.
//
// And the edition-2024 error for `println!("{LAUNCH_COUNT}")`:
//
// error: creating a shared reference to mutable static
//   = note: it's undefined behavior if the static is mutated while the
//           shared reference lives
//
// `static mut` is the one superpower you should never reach for: any thread
// can write it at any time, so a reference to it is unsound by construction.
// Use an atomic, or a Mutex, or `&raw const` as above.
