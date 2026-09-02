# Concept 43 · Custom error types — Under the hood

> Pair: [Use it](use-it.md) · **Under the hood** (you are here)
> Track: [From-Zero: Rust](../README.md)

## The short version
A `Result<T, E>` is an enum, and [Concept 23](../23-result/use-it.md) already told you what that costs: one shared slot, sized for the larger of `T` and `E`, plus a tag when there is no niche to hide it in. Everything in this lesson follows from that one sentence, and it has a consequence that catches people out:

> **Every success pays for the largest failure.**

`Ok(21)` is an `i32` in a slot big enough to hold your worst error variant. The `Result` is returned by value, so that width is copied into the caller's frame on every call — the millions that succeed as well as the handful that don't.

## Measuring it
```rust
#[derive(Debug)]
enum ReadingError {
    NotANumber(ParseIntError),
    OutOfRange { value: i32, limit: i32 },
    SensorOffline { name: String },
}
```

```
ParseIntError                1 bytes
String                      24 bytes
ReadingError                24 bytes
Result<i32, ReadingError>   24 bytes
Result<i32, Box<dyn Error>> 16 bytes
```

Read those in order, because each one is a small surprise.

`ParseIntError` is **1 byte**. It is an enum of a few kinds — empty, invalid digit, too large — with no payload. All that "invalid digit found in string" text is a `match` in its `Display` impl, generated at print time, not stored.

`ReadingError` is **24**, entirely because of `SensorOffline { name: String }`. A `String` is the 24-byte pointer/length/capacity header from [Concept 17](../17-vec/use-it.md), and the enum's shared slot must fit the widest variant. `NotANumber` needs 1 byte and `OutOfRange` needs 8, and neither of them gets to be smaller for it.

`Result<i32, ReadingError>` is also **24** — no tag added. This is the niche optimisation from [Concept 15](../15-option/use-it.md), working three levels deep: `String`'s pointer can never be null, so there are spare bit patterns inside the error to encode both which variant it is *and* whether the whole thing is `Ok` or `Err`. Rust found room in the gaps.

And `Result<i32, Box<dyn Error>>` is **16**, which is the interesting one.

## What `Box<dyn Error>` actually is
Two words, exactly the fat pointer from [Concept 21](../21-trait-objects/use-it.md):

```
Box<dyn Error>  =  [ data pointer ][ vtable pointer ]
                          8              8
```

The first word points at the error value on the heap. The second points at a table of function pointers for *that concrete type* — where its `Display::fmt` lives, where its `Debug::fmt` lives, where its `source` lives. Printing a boxed error is one hop through the vtable and then a call, the same dynamic dispatch you met with `Box<dyn Greet>`.

So a boxed error is 16 bytes in the return slot no matter how fat the actual error is — the bulk moved to the heap. And the allocation only happens when a call actually **fails**. On the success path there is no heap traffic at all, because `Ok(21)` never constructs the box.

That is a genuinely good trade, and it is worth seeing where the pieces land. A typed enum keeps everything inline: no allocation ever, no pointer chasing, exhaustive matching, but every success carries the width. A boxed error is a fixed 16 bytes with an allocation on the failure path only, and the caller loses the ability to `match`. Neither is "the fast one" — they are fast in different places.

## Why `std::io::Error` is 8 bytes
```
std::io::Error              8 bytes
Result<(), std::io::Error>  8 bytes
```

An `io::Error` can hold an OS error number, or a `Box<dyn Error + Send + Sync>` from a custom error, or a plain kind. That is obviously more than eight bytes of possibility. It is 8 because **std boxes it deliberately**, packing the whole thing behind one pointer and stealing the low bits of that pointer to record which of the shapes it is.

They did that on purpose, for exactly the reason above: `io::Error` is the failure arm of nearly every I/O call in the language, and `Result<(), io::Error>` is returned by `write!` in a hot loop. Fattening it would tax every successful write in every Rust program. So the error's data is the thing that goes indirect, and the pointer is what travels.

This is the design rule the numbers are pointing at. When you write your own error type and one variant is far bigger than the rest — a `String`, a nested struct, a backtrace — box **that variant** rather than widening the whole enum:

```rust
enum ReadingError {
    NotANumber(ParseIntError),                     // 1 byte
    OutOfRange { value: i32, limit: i32 },         // 8 bytes
    SensorOffline(Box<OfflineDetails>),            // 8 bytes, however big the details are
}
```

Clippy has a lint for this, `result_large_err`, and it fires around 128 bytes.

## Where `From` goes at run time
`?` is not a function call. It is the desugaring from [Concept 24](../24-question-mark/use-it.md), and `From::from` sits inside the `Err` arm:

```rust
match result {
    Ok(value) => value,
    Err(error) => return Err(From::from(error)),
}
```

`From` is a trait with a generic parameter, so the call is **monomorphized and statically dispatched** ([Concept 20](../20-traits/use-it.md)) — the compiler knows at the call site that it is converting a `ParseIntError` into a `ReadingError`, jumps straight to your impl, and typically inlines it away entirely. A `From` impl that just wraps a value in a variant compiles to writing the tag and moving the bytes. There is no lookup, no registry, no runtime cost for the conversion.

The blanket impl that makes `Box<dyn Error>` swallow everything is the same machinery:

```rust
impl<E: Error + Send + Sync + 'static> From<E> for Box<dyn Error + Send + Sync> { ... }
```

One impl in std, covering every error type that will ever exist, resolved at compile time per call site. That is why `?` "just worked" in the first exercise without you writing anything.

The one cost that *is* real: converting into a boxed error allocates. On the error path, which is the path where a few hundred nanoseconds do not matter.

## The chain, and `'static`
```rust
fn source(&self) -> Option<&(dyn Error + 'static)>
```

Two things are hiding in that signature.

`Option<&dyn Error>` is 16 bytes and needs no tag — a reference can never be null, so `None` is the all-zeros pattern. The niche optimisation again, in the standard library's own API.

And `+ 'static` is a **lifetime bound** ([Concept 25](../25-lifetimes/use-it.md)), which here means the error type behind the reference contains no borrowed data of its own. It is what lets an error be passed up through arbitrarily many frames, boxed, stored, and printed long after the function that produced it returned. An error that borrowed from the string it was parsing could not survive that trip, so std requires errors not to.

## Predict the memory
```rust
use std::error::Error;
use std::num::ParseIntError;

#[derive(Debug)]
enum Small {
    Parse(ParseIntError),
    TooBig { value: i32 },
}

#[derive(Debug)]
enum Large {
    Parse(ParseIntError),
    Offline { name: String },
}

fn main() {
    println!("{}", size_of::<Small>());
    println!("{}", size_of::<Result<i32, Small>>());
    println!("{}", size_of::<Large>());
    println!("{}", size_of::<Result<i32, Box<Large>>>());
    println!("{}", size_of::<Result<u8, Box<dyn Error>>>());
}
```

Before running it: which of the five numbers are equal to each other, and which one changes if you add a fourth variant to `Large` holding a `[u8; 64]`?

<details>
<summary>Answer</summary>
<p><code>8</code>, <code>8</code>, <code>24</code>, <code>16</code>, <code>16</code>.</p>
<p><code>Small</code> is 8: its widest payload is <code>TooBig</code>'s <code>i32</code> at 4 bytes, and the tag plus alignment to 4 rounds the whole thing to 8. <code>Result&lt;i32, Small&gt;</code> is <strong>also 8</strong> — no growth. Both arms need 4 bytes of payload, and <code>Small</code> has spare tag values left over, so <code>Ok</code>/<code>Err</code> is encoded in the gap rather than in a new byte.</p>
<p><code>Large</code> is 24, set entirely by <code>String</code>. But <code>Result&lt;i32, Box&lt;Large&gt;&gt;</code> is <strong>16</strong>: boxing the error replaces those 24 bytes with an 8-byte pointer, the <code>i32</code> needs 4, alignment rounds to 8, and the <code>Ok</code>/<code>Err</code> tag goes in the box pointer's null niche — 16 total.</p>
<p><code>Result&lt;u8, Box&lt;dyn Error&gt;&gt;</code> is 16 as well, but for a different reason worth noticing: the <code>Box&lt;dyn Error&gt;</code> half is already 16 (two words), and the <code>u8</code> and the tag both fit in the niche, so the payload is free. Two identical numbers, two different stories.</p>
<p>The fourth variant with <code>[u8; 64]</code> changes <code>Large</code> to 72 and would change <code>Result&lt;i32, Large&gt;</code> with it — but <code>Result&lt;i32, Box&lt;Large&gt;&gt;</code> stays at 16, because a pointer's size does not depend on what it points at. That is the whole argument for boxing a fat variant, in one line.</p>
</details>

## Next
- The macro-phases continue with **crates, Cargo and testing** — where the module tree from [Concept 42](../42-modules/use-it.md) grows a boundary that other people's code sits on the far side of, and where the error types above finally get a `#[test]` proving they say what you think they say.
