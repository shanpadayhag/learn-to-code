# Concept 43 · Custom error types — Use it

> Pair: **Use it** (you are here) · [Under the hood](under-the-hood.md)
> Track: [From-Zero: Rust](../README.md) · Previous: [Concept 42](../42-modules/use-it.md)

## The idea
[Concept 23](../23-result/use-it.md) gave you `Result<T, E>`, and every example filled `E` in with an error somebody else wrote — usually `ParseIntError`. That works right up to the moment a function can fail in more than one way:

```rust
fn parse_reading(text: &str) -> Result<i32, ???> {
    let value: i32 = text.trim().parse()?;   // fails with ParseIntError
    if value > 60 {
        return Err(???);                     // fails because 95C is nonsense
    }
    Ok(value)
}
```

The second failure has no type to be. It is not a parse error; the parse went fine. `ParseIntError` cannot be constructed by you anyway — its insides are private, exactly as [Concept 42](../42-modules/use-it.md) would predict.

So you need an error type of your own. And the question underneath this whole lesson is not "how do I make one" — it is **what does the caller need in order to do something other than give up?**

![Three panels: a Box dyn Error arriving as one printable message the caller cannot tell apart, beside an enum with three named variants each mapping to a different reaction; the ? operator as a funnel where ParseIntError, io::Error and Utf8Error all pass through From::from into one ReadingError; and the sizes — Result with a 24-byte enum error that every Ok also pays, versus a 16-byte boxed error whose bulk lives on the heap, and std::io::Error at 8 bytes because std boxes it deliberately](diagrams/error-types.svg)

## The quick answer: `Box<dyn Error>`
There is a type that means *any error at all*: a trait object, exactly as in [Concept 21](../21-trait-objects/use-it.md), over the standard `Error` trait.

```rust
use std::error::Error;

fn parse_reading(text: &str) -> Result<i32, Box<dyn Error>> {
    let trimmed = text.trim();

    if trimmed == "offline" {
        return Err("sensor is offline".into());
    }

    let value: i32 = trimmed.parse()?;

    if value > 60 {
        return Err(format!("{value}C is above the 60C limit").into());
    }

    Ok(value)
}
```

Everything here just works, and it is worth being precise about *why*, because it is the same mechanism twice. `"sensor is offline".into()` works because std implements `From<&str> for Box<dyn Error>`. And `trimmed.parse()?` works — despite `parse` failing with a `ParseIntError` while the function returns `Box<dyn Error>` — because `?` [inserts a `From::from`](../24-question-mark/use-it.md), and std implements `From<E> for Box<dyn Error>` for every `E` that is an `Error`. One `?`, any error type, no conversion code from you.

`main` can return it too, which is the fastest way to stop writing `.unwrap()`:

```rust
fn main() -> Result<(), Box<dyn Error>> {
    let reading = parse_reading("34")?;
    println!("{reading}C");
    Ok(())
}
```

Now look at what the caller gets:

```rust
match parse_reading(line) {
    Ok(value) => println!("{value}C"),
    Err(error) => println!("failed: {error}"),
}
```

That `Err` arm is the entire vocabulary. It can print the error. It cannot ask *which* failure happened, so it cannot retry the offline sensor, clamp the out-of-range value, and skip the typo — three sensible, different responses that are now indistinguishable. You could match on the text of the message, and you must not: that turns a human sentence into an API, and the day someone improves the wording your logic silently breaks.

`Box<dyn Error>` is the right answer for `main`, for scripts, for prototypes, and for anywhere the only plan is "report it and stop." It is the wrong answer the moment a caller wants to *respond*.

## The real answer: an enum
You already have the tool. Back in [Concept 14](../14-enums/use-it.md): a struct is AND, an enum is OR. "This failed for reason A **or** reason B **or** reason C" is an enum, and it always was.

```rust
use std::num::ParseIntError;

#[derive(Debug)]
enum ReadingError {
    NotANumber(ParseIntError),
    OutOfRange { value: i32, limit: i32 },
    SensorOffline { name: String },
}
```

Two design choices worth naming. Each variant **carries the data a caller needs to act** — `OutOfRange` hands over the value and the limit so the caller can clamp without re-deriving anything. And `NotANumber` **keeps the original error** rather than discarding it, which is what will let you show a full chain in a moment.

`#[derive(Debug)]` is required, not optional: `.unwrap()` and `?` in tests both need to print the error, and the `Error` trait demands it.

## The three impls that make it an error
An enum is not yet an error; it is an enum. Rust asks for three things, and each one answers a different question.

**`Display` — what does a human read?**

```rust
use std::fmt;

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
```

`Display` is what `{}` prints; `Debug` (derived) is what `{:?}` prints. The convention for error messages is a lowercase fragment with no trailing period, because callers wrap them: `"failed to read sensor: {error}"` should read as one sentence.

**`Error` — is this an error, and what caused it?**

```rust
use std::error::Error;

impl Error for ReadingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ReadingError::NotANumber(cause) => Some(cause),
            _ => None,
        }
    }
}
```

The trait itself requires nothing — `impl Error for ReadingError {}` compiles, as long as `Debug` and `Display` are already there. What it *gives* you is membership: your type can now be boxed as `Box<dyn Error>`, accepted by anything generic over errors, and asked for its cause.

`source` is that cause: the error underneath this one. Note the return type is a trait object, and it has to be — an error chain is a linked list of unrelated types, and `dyn` is the only thing that can hold "whatever came next." Walking it gives the full story:

```
reading was not a number
caused by: invalid digit found in string
```

**`From` — how does `?` convert into this?**

```rust
impl From<ParseIntError> for ReadingError {
    fn from(cause: ParseIntError) -> Self {
        ReadingError::NotANumber(cause)
    }
}
```

This is the one that turns four lines back into one character. Remember the desugaring from [Concept 24](../24-question-mark/use-it.md):

```rust
match result {
    Ok(value) => value,
    Err(error) => return Err(From::from(error)),
}
```

`From::from` was always in there. With `Box<dyn Error>` std supplied the impl; for your own type, you supply it. Write one `From` per incoming error type and every `?` in the function converts silently:

```rust
fn parse_reading(text: &str) -> Result<i32, ReadingError> {
    let trimmed = text.trim();

    if trimmed == "offline" {
        return Err(ReadingError::SensorOffline { name: String::from("roof-2") });
    }

    let value: i32 = trimmed.parse()?;   // ParseIntError -> ReadingError, via From

    if value > 60 {
        return Err(ReadingError::OutOfRange { value, limit: 60 });
    }

    Ok(value)
}
```

Delete the `From` impl and that `?` stops compiling, with the compiler naming the exact missing piece: *the trait `From<ParseIntError>` is not implemented for `ReadingError`*.

## And now the caller can act
```rust
match parse_reading(line) {
    Ok(value) => println!("{value}C"),
    Err(ReadingError::NotANumber(cause)) => println!("skipping, bad text ({cause})"),
    Err(ReadingError::OutOfRange { value, limit }) => println!("clamping {value} to {limit}"),
    Err(ReadingError::SensorOffline { name }) => println!("retrying {name} in 30s"),
}
```

Three failures, three different responses — the thing that was impossible with a boxed error. And because `match` is exhaustive ([Concept 16](../16-match/use-it.md)), the day you add a fourth variant every caller that needs updating stops compiling and tells you where it is. That is the real difference between the two designs: a message is checked by a human reading logs, a variant is checked by the compiler.

## Which one, when
| you are writing | use | because |
|---|---|---|
| `main`, a script, a prototype | `Box<dyn Error>` | nothing will respond to the failure anyway |
| a library, or any function whose caller must react | an enum | the caller can `match`, and the compiler polices it |
| an app with layers | an enum per layer, `From` between them | each layer speaks its own vocabulary, `?` translates at the boundary |

The middle row is why libraries almost always define their own error enum: they cannot know what their callers will want to do, so they refuse to throw the information away.

One honest note about the boilerplate. Three impls per error type, plus a `From` per source, is a real amount of typing, and in practice most Rust projects reach for a crate that generates it — `thiserror` derives `Display`, `Error` and `From` from attributes on the enum, and `anyhow` provides a better `Box<dyn Error>` with a built-in chain and backtrace. Both are worth knowing about, and neither teaches you anything new: they generate exactly the code above. Write it by hand once, and every `#[derive(Error)]` you meet afterwards is transparent rather than magic.

> Quick reference: [custom error types](../../../languages/rust.md#custom-errors) in the handbook. See also [`Result`](../../../languages/rust.md#result) and [`?`](../../../languages/rust.md#question-mark).

## Exercises
```bash
rustc --edition 2024 1-solution.rs && ./1-solution
```

1. **The loose version** — [starter](exercises/1-starter.rs) · [solution](exercises/1-solution.rs). Write `parse_reading` returning `Box<dyn Error>`, watch `?` bridge a `ParseIntError` with no help from you, then try to react differently to the three failures in the caller and find out you cannot. The last step asks you to account for `size_of::<Result<i32, Box<dyn Error>>>()`.
2. **The typed version** — [starter](exercises/2-starter.rs) · [solution](exercises/2-solution.rs). Same program with a `ReadingError` enum, all three impls, and a `From<ParseIntError>`. Write the `?` line *before* the `From` impl so you meet the error that names it, then walk the `source()` chain, then compare the five `size_of` numbers at the end and work out who pays for the widest variant.

## Next
- Why a `Result` is as wide as its widest arm and success pays for failure, why `std::io::Error` is only 8 bytes when it clearly holds more than that, what the two halves of a `Box<dyn Error>` actually point at, and why the error path is the one place an allocation is nearly free: [Under the hood](under-the-hood.md).
