// The precise answer: your own enum, one variant per way this can fail, each
// carrying the data that explains it. Same program as exercise 1 — but now the
// caller can tell the failures apart.
//
// Run with:  rustc --edition 2024 2-starter.rs && ./2-starter

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

const LIMIT: i32 = 60;

// 1. Define the error. Derive Debug (the programmer's view); the variants hold
//    what a caller needs to act:
//
//    #[derive(Debug)]
//    enum ReadingError {
//        NotANumber(ParseIntError),                  // keep the cause
//        OutOfRange { value: i32, limit: i32 },
//        SensorOffline { name: String },
//    }

// 2. impl fmt::Display for ReadingError — one sentence per variant, matching
//    on self. Convention: lowercase, no trailing period, because callers wrap
//    it ("failed to read sensor: {error}").

// 3. impl Error for ReadingError. The body can be empty, but implement
//    source() to hand back the cause:
//
//    fn source(&self) -> Option<&(dyn Error + 'static)> {
//        match self { ReadingError::NotANumber(cause) => Some(cause), _ => None }
//    }
//
//    Note the return type: an Option of a TRAIT OBJECT. The chain is a linked
//    list of unrelated error types, so `dyn` is the only thing that can hold it.

// 4. impl From<ParseIntError> for ReadingError, wrapping it in NotANumber.
//    This one impl is what makes `?` work in step 5. Write step 5 first
//    WITHOUT it and read the error — the compiler names the missing trait.

// 5. Write parse_reading(text: &str) -> Result<i32, ReadingError> with the
//    same three failures as exercise 1: "offline" -> SensorOffline, a bad
//    parse via `?`, and above LIMIT -> OutOfRange.

// 6. In main, loop over the same five inputs and match on the result — but now
//    with FOUR arms: Ok, NotANumber, OutOfRange, SensorOffline. Do something
//    different in each (skip / clamp / retry). This is the whole payoff.

// 7. Walk the chain: on a failed "warm", print the error, then loop
//    `let mut cause = error.source(); while let Some(inner) = cause { ... }`
//    printing each level. You should see your sentence, then the ParseIntError
//    underneath it.

// 8. Print size_of for ParseIntError, String, ReadingError,
//    Result<i32, ReadingError> and Result<i32, Box<dyn Error>>. Predict all
//    five first. The one to explain is why ReadingError is so much bigger than
//    its smallest variant — and who ends up paying for that.

fn main() {
    // your code here
}
