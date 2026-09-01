// `unsafe` does NOT switch off the borrow checker. It unlocks exactly five
// extra abilities and changes nothing else. Use three of them here, then
// prove to yourself that the ordinary rules never stopped running.
//
// Run with:  rustc --edition 2024 1-starter.rs && ./1-starter

// 1. A global that can change. This is superpower 3, and the one you should
//    never actually reach for — see step 6.
// static mut LAUNCH_COUNT: u32 = 0;

// 2. Superpower 2: an unsafe fn that bumps the counter.
//
// unsafe fn record_launch() {
//     unsafe { LAUNCH_COUNT += 1; }
// }
//
//    Note the SECOND unsafe block inside. `unsafe` on the signature restricts
//    the CALLER; since edition 2024 it does not make the body permissive.

// 3. A safe function that reads the counter back.
//
// fn launches_so_far() -> u32 {
//     unsafe { *(&raw const LAUNCH_COUNT) }
// }
//
//    `&raw const` takes the address without ever making a reference. Try
//    writing `unsafe { LAUNCH_COUNT }` in a println! instead and read the
//    error you get — that is step 6.

fn main() {
    // 4. Call record_launch three times inside one unsafe block, then print
    //    launches_so_far(). Note that the CALLER of launches_so_far writes no
    //    unsafe at all: the danger was wrapped up and put away.

    // 5. Superpower 1, borrowed from the library: `get_unchecked` skips the
    //    bounds check. Read the same element two ways and compare them.
    //
    //    let readings = vec![12, 47, 3, 91];
    //    let checked = readings[2];
    //    let unchecked = unsafe { *readings.get_unchecked(2) };
    //
    //    Then total the whole vector with get_unchecked in a loop.

    // 6. THE POINT OF THE EXERCISE. Try to break an ordinary rule inside an
    //    unsafe block:
    //
    //    let mut owner = 5;
    //    let borrowed = &owner;
    //    unsafe {
    //        owner += 1;
    //        println!("{borrowed}");
    //    }
    //
    //    Predict what happens before you compile. You should get TWO messages
    //    — one error and one warning — and the warning is the interesting
    //    one. Work out what it is telling you about what `unsafe` is for.

    // 7. Finally, print `LAUNCH_COUNT` directly in a println! inside an unsafe
    //    block. It is a hard error in edition 2024. Read why, and you will
    //    understand why step 3 goes through `&raw const`.
}
