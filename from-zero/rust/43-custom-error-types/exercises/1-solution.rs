// The loose answer: Box<dyn Error>. Any error at all can be poured into it, so
// `?` works across error types you never planned for, and `main` can return one.
//
// The cost is at the other end: the caller receives "an error" with no shape.
// It can print it. It cannot ask which failure happened.
//
// Run with:  rustc --edition 2024 1-solution.rs && ./1-solution

use std::error::Error;

const LIMIT: i32 = 60;

// `Box<dyn Error>` means: some value, somewhere on the heap, that implements
// the Error trait. Exactly the trait object from Concept 21 — a fat pointer,
// data pointer plus vtable pointer.
fn parse_reading(text: &str) -> Result<i32, Box<dyn Error>> {
    let trimmed = text.trim();

    if trimmed == "offline" {
        // A &str converts into a Box<dyn Error> too — std implements it. Handy,
        // and the reason so much early Rust code carries string errors around.
        return Err("sensor is offline".into());
    }

    // `?` on a ParseIntError. The function returns Box<dyn Error>, not
    // ParseIntError, and it still compiles — because `?` inserts a conversion,
    // and std provides From<E> for Box<dyn Error> for every E that is an Error.
    let value: i32 = trimmed.parse()?;

    if value > LIMIT {
        return Err(format!("{value}C is above the {LIMIT}C limit").into());
    }

    Ok(value)
}

fn main() {
    for line in ["21", "  34  ", "warm", "95", "offline"] {
        match parse_reading(line) {
            Ok(value) => println!("{line:>8} -> {value}C"),

            // Everything the caller can do with the error is on this line.
            // It knows the failure had a message. It cannot know which failure
            // it was, so it cannot retry the offline sensor and give up on the
            // typo — the two are the same shape now.
            Err(error) => println!("{line:>8} -> failed: {error}"),
        }
    }

    println!();
    println!("Result<i32, Box<dyn Error>> is {} bytes", size_of::<Result<i32, Box<dyn Error>>>());
    println!("  a fat pointer (data + vtable), and the tag hides in the null niche");
    println!("  nothing is allocated unless a call actually fails");
}

//       21 -> 21C
//     34   -> 34C
//     warm -> failed: invalid digit found in string
//       95 -> failed: 95C is above the 60C limit
//  offline -> failed: sensor is offline
//
// Result<i32, Box<dyn Error>> is 16 bytes
//   a fat pointer (data + vtable), and the tag hides in the null niche
//   nothing is allocated unless a call actually fails
