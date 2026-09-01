// The loose answer: Box<dyn Error>. Any error at all can be poured into it, so
// `?` works across error types you never planned for, and `main` can return one.
//
// Write it, enjoy how easy it is, then look hard at what the CALLER can do with
// what it gets back. That gap is what exercise 2 fixes.
//
// Run with:  rustc --edition 2024 1-starter.rs && ./1-starter

use std::error::Error;

const LIMIT: i32 = 60;

// 1. Write:
//
//    fn parse_reading(text: &str) -> Result<i32, Box<dyn Error>>
//
//    `Box<dyn Error>` is the trait object from Concept 21: some value on the
//    heap that implements Error, reached through a fat pointer.

// 2. Inside it, trim the text. If it is exactly "offline", return
//    Err("sensor is offline".into()). A &str converts into a Box<dyn Error>
//    because std implements it — which is why so much early Rust carries
//    string errors around.

// 3. Then parse it:  let value: i32 = trimmed.parse()?;
//
//    Stop and notice this compiles. `parse` fails with a ParseIntError, your
//    function returns Box<dyn Error>, and `?` bridges them on its own. Work
//    out which of Concept 24's two jobs is doing that. (It is the `From::from`
//    half — std has a From<E> for Box<dyn Error> covering every Error.)

// 4. If the value is above LIMIT, return
//    Err(format!("{value}C is above the {LIMIT}C limit").into()).
//    Otherwise Ok(value).

// 5. In main, loop over ["21", "  34  ", "warm", "95", "offline"], match on
//    the result, and print the Ok value or the error.

// 6. Now the important part. In the Err arm, TRY to react differently to the
//    three kinds of failure: retry the offline sensor, clamp the too-high
//    reading, skip the typo. You will find you cannot — not without matching
//    on the text of the message, which is not a thing you should ever do.
//    Write down what you would need instead. That is exercise 2.

// 7. Print size_of::<Result<i32, Box<dyn Error>>>() and account for the
//    number: what are the two halves of a fat pointer, and where did the
//    Ok/Err tag go?

fn main() {
    // your code here
}
