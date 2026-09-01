// The precise answer: your own enum. Now every way this function can fail is
// a named variant carrying the data that explains it, and the caller can match
// on which one happened and react differently to each.
//
// Three impls turn a plain enum into an error:
//   Debug    (derived)  — the programmer's view
//   Display             — the human sentence
//   Error               — "I am an error", plus the chain via source()
// and one more, From, is what makes `?` convert for you.
//
// Run with:  rustc --edition 2024 2-solution.rs && ./2-solution

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

const LIMIT: i32 = 60;

#[derive(Debug)]
enum ReadingError {
    // Wraps the error that caused this one. Keeping it is what lets source()
    // hand back the chain instead of losing the original reason.
    NotANumber(ParseIntError),
    OutOfRange { value: i32, limit: i32 },
    SensorOffline { name: String },
}

// What a human reads. One sentence, lowercase, no trailing period — the
// convention, because callers wrap it: "failed to read sensor: {error}".
impl fmt::Display for ReadingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReadingError::NotANumber(_) => write!(formatter, "reading was not a number"),
            ReadingError::OutOfRange { value, limit } => {
                write!(formatter, "{value}C is above the {limit}C limit")
            }
            ReadingError::SensorOffline { name } => write!(formatter, "sensor {name} is offline"),
        }
    }
}

impl Error for ReadingError {
    // The link to whatever caused this. Callers walk it to print a full chain;
    // returning None means "this is where the trail starts".
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ReadingError::NotANumber(cause) => Some(cause),
            _ => None,
        }
    }
}

// THIS is what makes `?` work. Delete this impl and `trimmed.parse()?` stops
// compiling: the usual message is E0277, "`?` couldn't convert the error to
// `ReadingError` ... the trait `From<ParseIntError>` is not implemented". (With
// `.parse()` specifically you get E0271 instead, because the error type comes
// from an associated type — same missing impl, different way of saying it.)
impl From<ParseIntError> for ReadingError {
    fn from(cause: ParseIntError) -> Self {
        ReadingError::NotANumber(cause)
    }
}

fn parse_reading(text: &str) -> Result<i32, ReadingError> {
    let trimmed = text.trim();

    if trimmed == "offline" {
        return Err(ReadingError::SensorOffline { name: String::from("roof-2") });
    }

    // One character. `?` unwraps on Ok, and on Err calls From::from — the impl
    // above — then returns early. Concept 24's desugaring, doing real work.
    let value: i32 = trimmed.parse()?;

    if value > LIMIT {
        return Err(ReadingError::OutOfRange { value, limit: LIMIT });
    }

    Ok(value)
}

fn main() {
    for line in ["21", "  34  ", "warm", "95", "offline"] {
        match parse_reading(line) {
            Ok(value) => println!("{line:>8} -> {value}C"),

            // The whole point. Three failures, three different reactions —
            // impossible in exercise 1, where they were all just "an error".
            Err(ReadingError::NotANumber(cause)) => {
                println!("{line:>8} -> skipping, bad text ({cause})");
            }
            Err(ReadingError::OutOfRange { value, limit }) => {
                println!("{line:>8} -> clamping {value} down to {limit}");
            }
            Err(ReadingError::SensorOffline { name }) => {
                println!("{line:>8} -> retrying sensor {name} in 30s");
            }
        }
    }

    // Walking the chain: our Display first, then whatever caused it.
    println!();
    if let Err(error) = parse_reading("warm") {
        println!("chain:");
        println!("  {error}");
        let mut cause = error.source();
        while let Some(inner) = cause {
            println!("  caused by: {inner}");
            cause = inner.source();
        }
    }

    println!();
    println!("ParseIntError               {:>2} bytes", size_of::<ParseIntError>());
    println!("String                      {:>2} bytes", size_of::<String>());
    println!("ReadingError                {:>2} bytes  (the widest variant wins)", size_of::<ReadingError>());
    println!("Result<i32, ReadingError>   {:>2} bytes  (every Ok pays it too)", size_of::<Result<i32, ReadingError>>());
    println!("Result<i32, Box<dyn Error>> {:>2} bytes  (the boxed version)", size_of::<Result<i32, Box<dyn Error>>>());
}

//       21 -> 21C
//     34   -> 34C
//     warm -> skipping, bad text (invalid digit found in string)
//       95 -> clamping 95 down to 60
//  offline -> retrying sensor roof-2 in 30s
//
// chain:
//   reading was not a number
//   caused by: invalid digit found in string
//
// ParseIntError                1 bytes
// String                      24 bytes
// ReadingError                24 bytes  (the widest variant wins)
// Result<i32, ReadingError>   24 bytes  (every Ok pays it too)
// Result<i32, Box<dyn Error>> 16 bytes  (the boxed version)
