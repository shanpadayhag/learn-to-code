// A reference is an address plus four promises: never null, always aligned,
// always pointing at a live value of the right type, and never aliasing a
// &mut. A raw pointer is the same address with all four promises stripped off.
//
// Making one is SAFE — an address is just a number. Reading through one is
// UNSAFE — believing a number is where the danger lives.
//
// Run with:  rustc --edition 2024 1-solution.rs && ./1-solution

use std::ptr;

fn main() {
    println!("a reference and a raw pointer are the same 8 bytes:");
    println!("  &i32        {}", size_of::<&i32>());
    println!("  *const i32  {}", size_of::<*const i32>());
    println!("  &[i32]      {}", size_of::<&[i32]>());
    println!("  *const [i32] {}", size_of::<*const [i32]>());

    let mut reading = 42;

    // `&raw mut` takes the address without ever creating a reference, so the
    // borrow checker never gets involved. Both pointers below come from this
    // one, which is what makes using them afterwards legitimate.
    let writable: *mut i32 = &raw mut reading;
    let readable: *const i32 = writable;

    println!();
    println!("two pointers, one address:");
    println!("  writable {writable:p}");
    println!("  readable {readable:p}");
    println!("  same address: {}", writable as usize == readable as usize);

    // Two *mut to the same place can coexist. Two &mut cannot — that is the
    // rule raw pointers exist to step around.
    let also_writable: *mut i32 = writable;

    unsafe {
        *writable = 100;
        println!();
        println!("wrote 100 through `writable`, read {} through `readable`", *readable);

        *also_writable += 1;
        println!("added 1 through a second *mut, now {}", *readable);
    }

    println!("and the original binding agrees: {reading}");

    // Pointer arithmetic counts in ELEMENTS, not bytes. .add(2) on a *const
    // i32 moves 8 bytes, because that is two i32s.
    let readings = [10, 20, 30, 40];
    let first: *const i32 = readings.as_ptr();
    unsafe {
        println!();
        println!("walking with .add():");
        for step in 0..readings.len() {
            println!("  element {step} at {:p} = {}", first.add(step), *first.add(step));
        }
    }

    // Creating a null pointer is safe. So is creating a dangling one. Neither
    // is dereferenced here, and that is the only reason this program is sound.
    let nothing: *const i32 = ptr::null();
    let dangling: *const i32 = {
        let temporary = 7;
        &raw const temporary
    };
    println!();
    println!("made a null pointer:     {nothing:p}  (is_null: {})", nothing.is_null());
    println!("made a dangling pointer: {dangling:p}  (its value died at the brace)");
    println!("neither was dereferenced — no unsafe block was needed to make them");
}

// a reference and a raw pointer are the same 8 bytes:
//   &i32        8
//   *const i32  8
//   &[i32]      16
//   *const [i32] 16
//
// two pointers, one address:
//   writable 0x7ff7b8a6dd14
//   readable 0x7ff7b8a6dd14
//   same address: true
//
// wrote 100 through `writable`, read 100 through `readable`
// added 1 through a second *mut, now 101
// and the original binding agrees: 101
//
// walking with .add():
//   element 0 at 0x7ff7b8a6ddf0 = 10
//   element 1 at 0x7ff7b8a6ddf4 = 20
//   element 2 at 0x7ff7b8a6ddf8 = 30
//   element 3 at 0x7ff7b8a6ddfc = 40
//
// made a null pointer:     0x0  (is_null: true)
// made a dangling pointer: 0x7ff7b8a6deb8  (its value died at the brace)
// neither was dereferenced — no unsafe block was needed to make them
//
// (Addresses differ every run — that is the stack, not a bug.)
//
// Now add `println!("{}", unsafe { *dangling });` and you have undefined
// behaviour: it will probably print 7, or some other number, or garbage,
// depending on what the compiler put in that stack slot next. Nothing warns.
// Nothing crashes. That silence is the whole reason the deref needs `unsafe`.
