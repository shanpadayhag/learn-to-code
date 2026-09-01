// A reference is an address plus four promises: never null, always aligned,
// always pointing at a live value of the right type, and never aliasing a
// &mut. A raw pointer is the same address with all four promises stripped off.
//
// Making one is SAFE — an address is just a number. Reading through one is
// UNSAFE — believing a number is where the danger lives.
//
// Run with:  rustc --edition 2024 1-starter.rs && ./1-starter

use std::ptr;

fn main() {
    // 1. Prove a reference and a raw pointer are the same size at runtime.
    //    Print size_of::<&i32>(), size_of::<*const i32>(), size_of::<&[i32]>()
    //    and size_of::<*const [i32]>(). Predict the four numbers first — one
    //    pair should surprise you, and the reason is in the lesson.

    // 2. Make two raw pointers to one variable.
    //
    //    let mut reading = 42;
    //    let writable: *mut i32 = &raw mut reading;
    //    let readable: *const i32 = writable;
    //
    //    `&raw mut` takes the ADDRESS without ever creating a reference, so
    //    the borrow checker never gets involved. Print both with {:p} and
    //    confirm they are the same address.

    // 3. Make a THIRD pointer, another *mut, to the same place. Two *mut can
    //    coexist; two &mut cannot. Write the &mut version and read the error
    //    if you want to see the rule you are stepping around.

    // 4. Inside one unsafe block: write 100 through `writable`, read it back
    //    through `readable`, then add 1 through the third pointer and read it
    //    again. Finally print `reading` itself and check it agrees.

    // 5. Pointer arithmetic counts ELEMENTS, not bytes.
    //
    //    let readings = [10, 20, 30, 40];
    //    let first: *const i32 = readings.as_ptr();
    //
    //    Loop 0..4 printing `first.add(step)` with {:p} and `*first.add(step)`.
    //    Look at the addresses: how far apart are they, and why that number?

    // 6. Make a null pointer with ptr::null(), and a dangling one by taking
    //    `&raw const` of a value inside a block that then ends. Print both.
    //    Note how much unsafe you needed to CREATE them. (None.)

    // 7. Now dereference the dangling one: println!("{}", unsafe { *dangling });
    //    You will probably get a plausible-looking number. Nothing warns and
    //    nothing crashes. Sit with that for a second — it is the whole reason
    //    the deref is the part that needs `unsafe`.
    let _ = ptr::null::<i32>();
}
