# Rust syntax handbook

A growing reference of Rust syntax, built from the solutions in this repo. Each
entry is explained once here and linked from wherever it's used. Assumes you know
how to program in *some* language — just not Rust yet.

## Contents
- [`use` declarations](#use)
- [`impl` blocks](#impl)
- [`pub fn` — functions](#pub-fn)
- [`Vec<T>` — growable arrays](#vec-type)
- [`vec![]` — the vector macro](#vec-macro)
- [`let` and `let mut`](#let-mut)
- [`HashMap<K, V>`](#hashmap)
- [`for` + `.iter()` + `.enumerate()`](#for-iter-enumerate)
- [`&` in patterns — destructuring a reference](#ref-pattern)
- [`if let` with `Option` / `Some`](#if-let)
- [`as` — numeric casts](#as-cast)
- [`Box<T>` — a pointer to the heap](#box)
- [`&mut` — mutable references](#mut-ref)
- [`while` loops](#while)
- [`Option::is_some`](#is-some)
- [`Option::take`](#option-take)
- [`Option::as_mut`](#option-as-mut)
- [`.unwrap()`](#unwrap)
- [`String` — an owned string](#string)
- [`.chars()` — iterate a string by character](#chars)
- [`usize` and underflow](#usize)
- [integer division — `/` truncates](#int-division)
- [`.max()` / `.min()` — pick the larger or smaller](#ord-max)
- [`struct` — defining your own type](#struct)
- [`enum` — one of several shapes](#enum)
- [`type` — type aliases](#type-alias)
- [`#[derive(Default)]` and `Self::default()`](#derive-default)
- [`match` expressions](#match)
- [`BTreeMap<K, V>` — a sorted map](#btreemap)
- [`.range(...)` on a `BTreeMap`](#btreemap-range)
- [`Option<T>` — a value that may be absent](#option)
- [`?` — the question-mark operator](#question-mark)
- [`let ... else` — bind or bail](#let-else)
- [`.entry(...).or_default()`](#entry-or-default)
- [closures — `|x| ...`](#closures)
- [iterator adapters — `.filter()`, `.map()`, `.collect()`, …](#iterator-adapters)
- [`impl Trait` in argument position](#impl-trait-arg)
- [lifetimes — `<'a>`](#lifetimes)
- [`format!` — building strings](#format)
- [`{:.1}` — format specifiers (decimals, width, …)](#format-spec)
- [`.to_owned()` / `.clone()` — making an owned copy](#to-owned-clone)
- [`Copy` types — values duplicated on assignment](#copy)
- [`&T` — shared references (borrowing)](#borrow)
- [slices — `&str` and `&[T]`](#slice)
- [generics — `<T>`](#generics)

## `use` declarations {#use}

**In one line:** pulls a name into scope so you can write `HashMap` instead of the
full `std::collections::HashMap` every time.

Rust's standard library is organized into modules with `::` as the separator
(like folders). `use std::collections::HashMap;` says "for the rest of this file,
`HashMap` means that one." Without it the code still works — you'd just have to
type the whole path at each use. Put `use` lines at the top of the file.

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md)

## `impl` blocks {#impl}

**In one line:** the place where you attach functions to a type.

`impl Solution { ... }` means "here are functions that belong to the `Solution`
type." LeetCode's Rust judge defines an empty `Solution` struct for you and calls
your method on it, which is why the harness code is wrapped in `impl Solution`.
For everyday Rust, `impl` is how you give a struct its methods.

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md)

## `pub fn` — functions {#pub-fn}

**In one line:** `fn` declares a function; `pub` makes it visible outside its
module.

`pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32>` reads as: a public
function named `two_sum`, taking a `Vec<i32>` and an `i32`, returning a `Vec<i32>`.
Parameter types come *after* the name with a colon; the return type follows `->`.
A function with no `->` returns nothing (the unit type `()`). The last expression
in the body — with no semicolon — is the return value, so explicit `return` is
only needed for early exits.

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md)

## `Vec<T>` — growable arrays {#vec-type}

**In one line:** a resizable list of values that all share one type `T`.

`Vec<i32>` is a vector of 32-bit signed integers. It's Rust's everyday "array that
can grow", living on the heap. The `<i32>` is a *type parameter* — `Vec<String>`,
`Vec<bool>`, etc. all work the same way. You index with `v[0]` and read its length
with `v.len()`.

Taught from zero — the ptr/len/cap header, doubling growth (amortized O(1)), and why a
regrow breaks old borrows — in [From-Zero Concept 17](../from-zero/rust/17-vec/use-it.md).

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md)

## `vec![]` — the vector macro {#vec-macro}

**In one line:** a shorthand for building a `Vec` from listed values.

`vec![earlier_index, current_index]` creates a two-element vector; `vec![]` creates
an empty one. The `!` marks it as a *macro* (code that expands into more code at
compile time), not a function — that's how it can take any number of arguments and
figure out the element type from them.

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md)

## `let` and `let mut` {#let-mut}

**In one line:** `let` declares a variable; by default it can't be reassigned, and
`mut` is what makes it changeable.

In Rust, variables are *immutable* unless you opt out. `let x = 5;` binds `x` once
and forever. `let mut x = 5;` lets you later do `x = 6;` or call methods that
mutate it. This default-immutable rule is a safety feature: the compiler stops you
from changing things you didn't mean to. We use `let mut index_of_seen_value`
because we insert into the map as we scan.

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md)

## `HashMap<K, V>` {#hashmap}

**In one line:** Rust's key → value store, where keys are type `K` and values type
`V`.

This is Rust's spelling of the [hash map](../glossary/hash-map.md) concept.
`HashMap<i32, i32>` maps integer keys to integer values. Create one with
`HashMap::new()` (needs `use std::collections::HashMap;` first). Key methods:
`.insert(key, value)` to store, and `.get(&key)` to look up — note `.get` takes a
*reference* to the key and returns an [`Option`](#if-let) (`Some(value)` if present,
`None` if not), because the key might be missing.

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md)

## `for` + `.iter()` + `.enumerate()` {#for-iter-enumerate}

**In one line:** loop over a collection, getting both each item and its position.

`for (i, &v) in numbers.iter().enumerate()` breaks down as:
- `.iter()` produces a sequence of *references* to the elements (it borrows the
  vector rather than consuming it, so `numbers` is still usable afterward).
- `.enumerate()` wraps each item into a `(index, item)` pair, counting from 0.
- `for (i, v) in ...` destructures that pair into two names each loop.

The result: `i` walks `0, 1, 2, ...` while `v` walks the values. (For why it's
`&v` and not `v`, see [`&` in patterns](#ref-pattern).)

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md)

## `&` in patterns — destructuring a reference {#ref-pattern}

**In one line:** an `&` on the *left* of `=` unwraps a reference, giving you the
value by copy instead of a pointer to it.

`.iter()` yields `&i32` (references to integers), but arithmetic like
`target - current_value` is cleaner on a plain `i32`. Writing the loop variable as
`&current_value` says "the thing I'm receiving is a reference — peel it off and
bind the value underneath." Since `i32` is cheaply copyable, this just copies the
number out. The same move appears in `if let Some(&earlier_index) = ...`, peeling
the reference that `.get()` hands back. Without the `&`, you'd be holding a `&i32`
and would have to dereference with `*` everywhere you used it.

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md)

## `if let` with `Option` / `Some` {#if-let}

**In one line:** run a block only when a value is present, and pull that value out
in the same line.

Rust has no `null`. A value that might be absent has type `Option<T>`, which is
either `Some(value)` or `None`. `.get()` on a map returns one of these.
`if let Some(&earlier_index) = index_of_seen_value.get(&needed_value) { ... }` means
"if the lookup returned `Some`, bind what's inside to `earlier_index` and run the
block; otherwise skip it." It's the concise alternative to a full `match` when you
only care about one case.

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md)

## `as` — numeric casts {#as-cast}

**In one line:** converts a number from one type to another, explicitly.

Rust never silently mixes number types, so `current_index` (a `usize`, the type
used for indexes and lengths) must be converted before it can sit in a `Vec<i32>`.
`current_index as i32` performs that conversion. `as` is the blunt cast for
primitives — quick and direct, though for conversions that could lose data Rust
also offers safer checked options elsewhere.

First seen in: [1. Two Sum](../problems/0001-two-sum/solution.rs.md)

## `Box<T>` — a pointer to the heap {#box}

**In one line:** a pointer that owns one value stored on the heap, used when a type
needs to point at "more of itself."

**What it is.** Most values live *inline* — right where they're declared. `Box<T>`
instead puts the `T` on the heap (the program's big pool of long-lived memory) and
keeps just a pointer to it. The `Box` owns that value: when the `Box` goes away, the
heap value is freed automatically.

**Why a [linked list](../glossary/linked-list.md) can't live without it.** A node
holds the *next* node. Picture writing it without a box:
```rust
struct ListNode { val: i32, next: Option<ListNode> }   // ❌ does not compile
```
To lay this out, the compiler must know a node's size. But a node contains a node,
which contains a node... forever — an infinitely large type. The error literally says
*"recursive type has infinite size."* A `Box` breaks the cycle: a pointer is a fixed,
known size no matter how big the thing it points to, so the next node lives elsewhere
on the heap and the node's size is finally knowable:
```rust
struct ListNode { val: i32, next: Option<Box<ListNode>> }   // ✅
```

**Trace the types.** `Box::new(ListNode::new(0))` allocates a `ListNode` on the heap
and hands back a `Box<ListNode>` that owns it. You rarely write `*` to reach inside —
field access like `node.val` and `result_tail.next` *auto-dereferences* through the
box for you.

**Why this way.** It's the smallest possible indirection that makes a self-referential
type have a finite size, while still owning its contents (no manual free, no garbage
collector). Pairing it with [`Option`](#if-let) — `Option<Box<ListNode>>` — gives the
two halves of a linked list: "a pointer to the next node" *or* "nothing, this is the
end," with no null pointers involved.

First seen in: [2. Add Two Numbers](../problems/0002-add-two-numbers/solution.rs.md)

## `&mut` — mutable references {#mut-ref}

**In one line:** a borrow you're allowed to *change* the value through — the
read-write counterpart to the read-only [`&`](#borrow).

**The two kinds of borrow.** Two Sum used `&`, a *shared* reference: you can look but
not touch. `&mut` is an *exclusive* reference: you can modify the value in place,
without taking ownership of it. The rule Rust enforces: at any moment a value can have
**many** `&` readers **or exactly one** `&mut` writer — never both at once. That
single-writer guarantee is what makes data races impossible.

**Here.** `let mut result_tail = &mut result_head;` borrows the head node mutably, so
we can grow the list *through* the borrow — `result_tail.next = Some(...)` writes into
the real node, not a copy. Later `result_tail = result_tail.next.as_mut().unwrap();`
re-points the borrow at the freshly added node so the next write lands at the new end.

**Without it.** With a plain `&result_head` the line `result_tail.next = Some(...)`
won't compile — you can't assign through a read-only borrow. The alternative would be
to pass *ownership* of the list around and hand it back each step, which is far
clumsier than borrowing it once and writing through the borrow.

**One confusing overlap.** `mut` shows up in two different roles. In `let mut x`, the
`mut` makes the *binding* reassignable (`x = ...` later). In `&mut x`, the `mut` makes
a *reference through which you can mutate* the pointed-to value. `let mut result_tail =
&mut result_head;` uses both: the binding is reassignable (we re-point it each loop)
*and* it's a mutable reference (we write through it).

First seen in: [2. Add Two Numbers](../problems/0002-add-two-numbers/solution.rs.md)
· the borrow rules taught in [From-Zero Concept 11](../from-zero/rust/11-mut-references-and-borrow-rules/use-it.md)

## `while` loops {#while}

**In one line:** repeat a block as long as a condition stays true.

`while condition { ... }` checks `condition` before each pass and stops the moment it's
false. Two Sum used a `for` loop because it walked a known sequence to its end. Here we
use `while` because we stop on a *dynamic* condition — "both lists are used up **and**
no carry is left" — not a fixed number of steps:
```rust
while first_digit.is_some() || second_digit.is_some() || carry != 0 { ... }
```

First seen in: [2. Add Two Numbers](../problems/0002-add-two-numbers/solution.rs.md)

## `Option::is_some` {#is-some}

**In one line:** asks an [`Option`](#if-let) "are you holding a value?" and answers
`true` or `false`.

`first_digit.is_some()` is `true` when `first_digit` is `Some(...)` and `false` when
it's `None`. It only *peeks* — it doesn't take the value out — which is exactly what a
loop condition wants: we check whether digits remain without disturbing them. (Its
mirror image is `.is_none()`.)

First seen in: [2. Add Two Numbers](../problems/0002-add-two-numbers/solution.rs.md)

## `Option::take` {#option-take}

**In one line:** rips the value out of an `Option`, leaves `None` in its place, and
hands you what was there.

**The problem it solves.** `first_digit` is an owned `Option<Box<ListNode>>`, and we
want the node inside to read its digit and step to `.next`. The obvious
`if let Some(node) = first_digit` *moves* `first_digit` into the match — and Rust then
considers `first_digit` used-up for the rest of the loop, so the next iteration's
`first_digit.is_some()` won't compile. We need the inside *without* destroying the
variable.

**What `.take()` does.** It swaps the slot to `None` and returns the old contents,
working through a `&mut`:
- before: `first_digit` is `Some(box)`
- `first_digit.take()` returns `Some(box)` **and** sets `first_digit` to `None`
- we match the returned `Some(node)`, then immediately overwrite the now-`None`
  `first_digit` with `node.next`

So the variable is always left in a valid state. If the list was already empty,
`.take()` returns `None`, the `if let` simply doesn't fire, and `first_digit` stays
`None` — precisely the "treat a missing digit as nothing" behavior we want.

**Without it.** You'd reach for `std::mem::replace(&mut first_digit, None)` by hand —
which is exactly what `.take()` is shorthand for.

First seen in: [2. Add Two Numbers](../problems/0002-add-two-numbers/solution.rs.md)

## `Option::as_mut` {#option-as-mut}

**In one line:** turns a `&mut Option<T>` into an `Option<&mut T>` — lets you reach a
mutable pointer to the value *inside* without removing it.

After `result_tail.next = Some(Box::new(...))`, we want to advance the tail to that
brand-new node. `.take()` would be wrong here — it would yank the node back out, the
opposite of what we want. `.as_mut()` instead borrows into the `Option`:
- `result_tail.next` is an `Option<Box<ListNode>>`
- `.as_mut()` gives `Option<&mut Box<ListNode>>` — a mutable peek, value left in place
- [`.unwrap()`](#unwrap) pulls out the `&mut Box<ListNode>` we re-point the tail to

So: `.take()` when you mean to *remove* the value, `.as_mut()` when you mean to *keep
it and borrow it*.

First seen in: [2. Add Two Numbers](../problems/0002-add-two-numbers/solution.rs.md)

## `.unwrap()` {#unwrap}

**In one line:** pulls the value out of a `Some` (or an `Ok`), and crashes the program
if it's `None` instead.

`.unwrap()` is the blunt way to get inside an [`Option`](#if-let): on `Some(x)` it
returns `x`, on `None` it panics. That makes it risky in general — a `None` you didn't
expect takes the whole program down. It's safe *here* only because the line right above
just set `result_tail.next = Some(...)`, so the value is provably present; we use
`.unwrap()` to say "I know this is `Some`." When you *can't* prove that, reach for
[`if let`](#if-let) or a `match`, which handle the `None` case instead of exploding.

First seen in: [2. Add Two Numbers](../problems/0002-add-two-numbers/solution.rs.md)

## `String` — an owned string {#string}

**In one line:** a growable, owned piece of text — the string type you get handed
when a function *owns* its text input.

Rust has two main string types, and the split trips up newcomers:
- `String` — owns its text on the heap, can grow and shrink. This is what
  `length_of_longest_substring(text: String)` receives: the function takes ownership
  of the whole string.
- `&str` — a *borrowed view* into text someone else owns (a "string slice"). A
  literal like `"abc"` is a `&str`.

Think of `String` as owning the notebook and `&str` as being allowed to read a page
of someone else's notebook. Here we only read `text`, so `&str` would also have worked —
but LeetCode's signature hands us an owned `String`, so that's what we take. We never
need to reach for the difference in this solution; we immediately walk it with
[`.chars()`](#chars).

Building and growing one: `String::from("Hi")` (or `"Hi".to_string()`) makes an owned
`String` from a literal; `String::new()` starts an empty one. `.push_str("...")` appends
text, `.push('c')` appends a single `char`, and `.len()` reports how many **bytes** it
currently holds. On the stack a `String` is just a small fixed-size handle — a pointer to
the heap, a length, and a capacity — which is why it isn't a [`Copy`](#copy) type.

First seen in: [3. Longest Substring Without Repeating Characters](../problems/0003-longest-substring-without-repeating-characters/solution.rs.md)
· grow/len methods in [From-Zero Concept 07](../from-zero/rust/07-the-heap-and-string/use-it.md)

## `.chars()` — iterate a string by character {#chars}

**In one line:** walks a string one *character* at a time, rather than one raw byte
at a time.

Rust text is stored as UTF-8, where one character can take several bytes. So Rust
makes you say *how* you want to walk it, and `.chars()` is the "give me whole
characters" choice.

**What types are flowing.** Trace `for (current_index, current_char) in text.chars().enumerate()`:
- `text` is a `String`
- `.chars()` yields `char` — each Unicode character, **by value** (a `char` is a
  cheap 4-byte `Copy` type, so you get your own copy, not a reference)
- `.enumerate()` wraps each into `(usize, char)`, counting from 0

So each loop item is a `(usize, char)`, unpacked into `current_index` and
`current_char`.

**Why no `&` peel here?** Compare with the [`.iter()` loop in Two Sum](#for-iter-enumerate),
where we wrote `&current_value` to strip a reference. That was needed because
`.iter()` yields *references* (`&i32`). `.chars()` is different — it yields owned
`char` values outright, so there's nothing to peel and the pattern is a plain
`current_char`. One fewer `&` to remember, purely because of what the iterator
produces.

**A word on the index.** `current_index` here counts **characters**, not bytes —
because `.enumerate()` numbers the items `.chars()` produces. That's exactly what we
want for measuring a substring's length in characters.

First seen in: [3. Longest Substring Without Repeating Characters](../problems/0003-longest-substring-without-repeating-characters/solution.rs.md)

## `usize` and underflow {#usize}

**In one line:** `usize` is Rust's *unsigned* integer for sizes and positions — it
can't go negative, and subtracting past zero **crashes** rather than wrapping to a
minus number.

Positions and lengths in Rust are `usize` (an unsigned integer: zero or above,
never negative). That's why `current_index` and `window_start` are `usize`. The
catch: because it can't represent `-1`, a subtraction like `a - b` where `b > a`
doesn't give a negative — in debug builds it **panics** (crashes), and in release
builds it silently wraps to a huge number. Either way it's a bug.

So `current_index - window_start + 1` is only safe because we can *prove*
`window_start` never passes `current_index`. And we can: `window_start` only ever
jumps to `previous_index + 1`, and `previous_index` is always an earlier position
than `current_index`, so `window_start ≤ current_index` at that line — the
subtraction is always `≥ 0`. When you subtract `usize` values, always check that the
left side can't dip below the right.

First seen in: [3. Longest Substring Without Repeating Characters](../problems/0003-longest-substring-without-repeating-characters/solution.rs.md)

## integer division — `/` truncates {#int-division}

**In one line:** when both sides of `/` are integers, Rust does *integer division*
— it throws the fractional part away, so `9 / 5` is `1`, not `1.8`.

Rust picks the kind of division from the **types**, not from what you meant. If both
operands are integers (`i32`, `u64`, `usize`, …), the result is an integer, and any
remainder is silently dropped — no rounding, no warning. So `9 / 5` is `1`, `7 / 2`
is `3`, and `1 / 2` is `0`. This bites hardest when the division is buried in a
formula: `celsius * (9 / 5) + 32` looks right, but `9 / 5` becomes `1` first, and the
whole thing collapses to `celsius + 32`.

To keep the decimal, make at least one side a **float** — either annotate the value
(`let celsius: f32 = ...`) or write the literals with a dot (`9.0 / 5.0`, which is
`1.8`):

```rust
let celsius: f32 = 100.0;
let fahrenheit = celsius * (9.0 / 5.0) + 32.0; // 212.0, not 132
```

Because `celsius` is `f32`, the `9.0 / 5.0` literals are inferred as `f32` too, so
the types line up with no cast needed. (Rust's *default* float, when nothing forces
a choice, is `f64`.) The same trap exists in C, Java, and Go — any language where
`int / int` stays an `int`.

First seen in: [Celsius → Fahrenheit](../challenges/celsius-to-fahrenheit/README.md)

## `.max()` / `.min()` — pick the larger or smaller {#ord-max}

**In one line:** `a.max(b)` returns whichever of `a` and `b` is bigger; `.min()`
returns the smaller.

Any two values that can be ordered (all the number types, for instance) support
`.max()` and `.min()` as methods. `longest = longest.max(current_index - window_start + 1)`
reads as "set `longest` to the bigger of its current value and the new window
width" — the standard way to keep a running maximum without an `if`.

**The "without it" version.** You could write it by hand:
```rust
let width = current_index - window_start + 1;
if width > longest {
    longest = width;
}
```
Same effect, three lines instead of one. `.max()` is just the tidy, idiomatic form
of that comparison, and it reads as exactly what it does.

First seen in: [3. Longest Substring Without Repeating Characters](../problems/0003-longest-substring-without-repeating-characters/solution.rs.md)

## `struct` — defining your own type {#struct}

**In one line:** bundles a few named values into one new type you can name and pass
around.

`impl Solution` in earlier problems used a type LeetCode handed us. Here we define
our own:
```rust
struct TimedValue {
    value: String,
    expires_at: Option<Timestamp>,
}
```
This says "a `TimedValue` is a `value` *and* an `expires_at`, together." You build one
by naming every field — `TimedValue { value: v, expires_at: None }` — and read a field
with a dot — `stored.value`. Fields are private to the defining module by default,
which is fine here since everything lives in one file. A struct with methods gets an
[`impl` block](#impl) (that's where `is_alive_at` attaches to `TimedValue`).

A struct owns its fields, and its fields sit next to each other in memory: a
`Copy`-only struct lives entirely on the stack, while a field like `String` keeps its
handle inline in the struct and its text on the heap. Moving the struct moves every
field with it.

First seen in: [In-Memory Database](../patterns/in-memory-database/solution.rs.md)
· taught from zero in [From-Zero Concept 13](../from-zero/rust/13-structs/use-it.md)

## `enum` — one of several shapes {#enum}

**In one line:** defines a type whose value is *exactly one* of a fixed list of variants —
the "or" to a [struct](#struct)'s "and."

```rust
enum Shape {
    Circle(f64),          // a variant can carry its own data
    Rectangle(f64, f64),  // different variants, different data
}
let s = Shape::Circle(2.0);   // build with Type::Variant(...)
```

A struct holds all its fields at once; an enum holds one variant at a time. Variants may
be fieldless (`enum Light { Red, Yellow, Green }`) or carry data — a tuple like
`Circle(f64)`, or named fields like `Move { x: i32, y: i32 }`. Construct with `::` and the
variant name.

**In memory** it's a **tag** (which variant) plus **one shared slot** sized for the
largest variant, so an enum is as big as its biggest variant plus the tag — never the sum.
A fieldless enum is just the tag (`Light` is 1 byte). Ownership carries over unchanged: if
any variant owns heap data it's a move type, otherwise it can be `Copy`.

To read one you [`match`](#if-let) on it, binding each variant's data into names. `Option`
and `Result` are just enums from the standard library.

First seen in: taught from zero in [From-Zero Concept 14](../from-zero/rust/14-enums/use-it.md)

## `type` — type aliases {#type-alias}

**In one line:** gives a long type a short, meaningful name — no new type, just a
nickname.

`type Record = BTreeMap<String, TimedValue>;` means "wherever I write `Record`, read
`BTreeMap<String, TimedValue>`." It's purely a readability tool: `Record` and the full
spelling are interchangeable to the compiler. We use it so signatures read as
`fn capture_live_fields(record: &Record, ...)` instead of dragging the whole nested map
type through every line. Distinct aliases (`Record` vs `SnapshotRecord`) also *document
intent* even though both are `BTreeMap`s.

First seen in: [In-Memory Database](../patterns/in-memory-database/solution.rs.md)

## `#[derive(Default)]` and `Self::default()` {#derive-default}

**In one line:** asks the compiler to write the "empty starting value" of a type for
you.

`#[derive(Default)]` on top of a struct auto-generates a `default()` constructor that
fills every field with *its* default (an empty `HashMap`, an empty `BTreeMap`, and so
on). Then:
```rust
pub fn new() -> Self {
    Self::default()
}
```
`Self` is a stand-in for "the type this `impl` is for" (`InMemoryDatabase`), so
`Self::default()` builds a fresh empty database. We expose it as `new()` by convention.
Without the derive you'd hand-write `InMemoryDatabase { records: HashMap::new(),
backups: BTreeMap::new() }` — fine now, but the derive keeps `new()` correct
automatically if you add a field later.

First seen in: [In-Memory Database](../patterns/in-memory-database/solution.rs.md)

## `match` expressions {#match}

**In one line:** picks a branch by comparing a value against several *patterns*, and
must cover every case.

[`if let`](#if-let) handles one case and ignores the rest; `match` is the full form
when you want to handle all of them:
```rust
match self.expires_at {
    Some(expiry) => timestamp < expiry,
    None => true,
}
```
Read it as "if there's a deadline, are we before it? if there's none, it's alive." The
compiler *forces* you to cover every variant of an [`Option`](#option) — leave off the
`None` arm and it won't compile — which is how `match` stops you forgetting a case.
Each arm is an expression, so the whole `match` evaluates to a value (here, the `bool`
the function returns).

Taught from zero — patterns (literals, ranges, `|`, bindings, `_`), exhaustiveness, and
the move-vs-borrow catch when binding a variant's data — in
[From-Zero Concept 16](../from-zero/rust/16-match/use-it.md).

First seen in: [In-Memory Database](../patterns/in-memory-database/solution.rs.md)

## `BTreeMap<K, V>` — a sorted map {#btreemap}

**In one line:** like a [`HashMap`](#hashmap), but it keeps its keys in **sorted
order**, so it can answer range and prefix questions.

This is Rust's spelling of the [sorted map](../glossary/sorted-map.md) concept, backed
by a balanced tree. Same everyday methods as `HashMap` — `.insert`, `.get`, `.remove`,
`.iter` — with two differences that matter:
- **Ordering:** `.iter()` yields entries **sorted by key**, always. That's why
  `scan_at` comes out sorted with no sort step.
- **Cost:** lookups are `O(log n)` instead of `HashMap`'s average `O(1)` — the price of
  keeping order.

We use it for the *inner* record (`BTreeMap<String, TimedValue>`) so fields stay sorted
for prefix scans, and for the *backup shelf* (`BTreeMap<Timestamp, Snapshot>`) so
"latest backup at or before T" is a fast [range](#btreemap-range) query. The outer map
stays a `HashMap` because it's only ever looked up by exact key.

First seen in: [In-Memory Database](../patterns/in-memory-database/solution.rs.md)

## `.range(...)` on a `BTreeMap` {#btreemap-range}

**In one line:** walks only the slice of the map between two bounds, instead of the
whole thing — the payoff for keeping keys sorted.

**What it does.** `map.range(a..)` iterates entries with key ≥ `a`, in order;
`map.range(..=b)` iterates entries with key ≤ `b`. Because a `BTreeMap` is sorted, it
jumps to the boundary in `O(log n)` and then walks — it never touches keys outside the
range.

**The two uses here, traced.**
- Prefix scan: `record.range(prefix.to_owned()..)` starts at the first field ≥ `prefix`,
  and [`.take_while(...)`](#iterator-adapters) stops as soon as a field no longer starts
  with `prefix`. Together that reads exactly the prefix block — `O(log F + M)`.
- Latest backup: `self.backups.range(..=timestamp_to_restore).next_back()` takes all
  backups at or before the target and `.next_back()` grabs the **last** one — i.e. the
  most recent. `.next_back()` works because a range is a double-ended iterator you can
  read from either end.

**The gotcha we hit — why `prefix.to_owned()`.** The natural `record.range(prefix..)`
where `prefix: &str` **does not compile**. Rust tries to compare using `&str`, then
needs `String: Borrow<&str>` (so it can view a stored `String` key as your bound's
type), and that trait impl doesn't exist — you get *"the trait `Borrow<&str>` is not
implemented for `String`."* The clean fix is to make the bound an owned `String` with
`prefix.to_owned()`, so both sides are `String` and the comparison just works. It costs
one small allocation, alongside the `Vec` the scan already builds — a fair price for a
line that reads plainly.

**Why bother instead of filtering.** `record.iter().filter(|(f, _)| f.starts_with(prefix))`
would also work — but it scans **all** `F` fields every time. The range walk touches
only the matches. On a large record that's the whole difference between `O(F)` and
`O(log F + M)`.

First seen in: [In-Memory Database](../patterns/in-memory-database/solution.rs.md)

## `Option<T>` — a value that may be absent {#option}

**In one line:** Rust's answer to `null` — a value is either `Some(thing)` or `None`,
and the compiler makes you handle both.

[`if let`](#if-let) already showed `Option` in a lookup; here it's also a *stored* and
*returned* type. `expires_at: Option<Timestamp>` means "a deadline **or** nothing"
(`None` = never expires). `get_at(...) -> Option<&str>` means "the value **or** nothing"
(`None` = absent or expired). Because there's no `null`, a missing value can't sneak
past you — you can't use the inner value without first opening the `Option` (via
[`match`](#match), [`if let`](#if-let), [`?`](#question-mark), or a method).

**`.map` on an `Option`.** `stored.expires_at.map(|expiry| expiry - timestamp)`
transforms the value *inside* a `Some`, leaving `None` untouched: `Some(25)` becomes
`Some(15)`, `None` stays `None`. It's the tidy way to say "if there is a deadline,
convert it to a remaining duration; if there isn't, there still isn't."

Taught from zero — why null is the problem, and how `Option` is just an enum — in
[From-Zero Concept 15](../from-zero/rust/15-option/use-it.md); the concept behind it lives
in [Null (and the billion-dollar mistake)](../glossary/null-and-the-billion-dollar-mistake.md).

First seen in: [In-Memory Database](../patterns/in-memory-database/solution.rs.md)

## `?` — the question-mark operator {#question-mark}

**In one line:** unwraps a `Some`/`Ok` and, on `None`/`Err`, *returns early* from the
whole function with that same empty result.

**Trace it.**
```rust
let stored = self.records.get(key)?.get(field)?;
```
- `self.records.get(key)` is `Option<&Record>`.
- `?` says: if that's `None`, **stop and return `None` from `get_at` right now**;
  otherwise hand back the `&Record` inside and keep going.
- `.get(field)?` does the same one level deeper.

So by the line after, `stored` is a plain `&TimedValue` — both "missing key" and
"missing field" have already been dealt with by bailing out. It only works because
`get_at` itself returns an `Option`, so there's a `None` for `?` to return.

**Without it.** You'd nest two matches:
```rust
let record = match self.records.get(key) {
    Some(r) => r,
    None => return None,
};
let stored = match record.get(field) {
    Some(s) => s,
    None => return None,
};
```
Six lines of ceremony for what `?` says in two characters. That "bail out on absence,
otherwise unwrap and continue" is the pattern `?` exists for.

First seen in: [In-Memory Database](../patterns/in-memory-database/solution.rs.md)

## `let ... else` — bind or bail {#let-else}

**In one line:** unwrap a value into a variable that stays in scope, and if it isn't
there, run an `else` block that must *leave* (return, break, continue).

**Trace it.**
```rust
let Some(record) = self.records.get_mut(key) else {
    return false;
};
record.remove(field);
```
- `self.records.get_mut(key)` is `Option<&mut Record>`.
- `let Some(record) = ... else { return false; }` says: if it's `Some`, bind the inside
  to `record` **and carry on with `record` usable below**; if it's `None`, run the
  `else`, which here returns `false`.

The key contrast with [`if let`](#if-let): with `if let`, the unwrapped value lives only
*inside* the `if` block, which pushes the rest of your logic one indent deeper each
time. `let ... else` unwraps and keeps the value at the **top level**, so the happy path
reads as a flat sequence instead of a staircase of nested blocks. That's why it fits the
"check a precondition, then get on with it" style of `delete_at`, `scan_by_prefix_at`,
and `restore`.

**The rule:** the `else` block must diverge — `return`, `break`, `continue`, or panic —
because after it, Rust needs `record` to definitely exist. An `else` that fell through
without leaving wouldn't compile.

First seen in: [In-Memory Database](../patterns/in-memory-database/solution.rs.md)

## `.entry(...).or_default()` {#entry-or-default}

**In one line:** looks up a key and, if it's missing, inserts a fresh empty value —
then hands you a mutable reference either way.

```rust
self.records.entry(key.to_owned()).or_default().insert(field, ...);
```
Reading it: `.entry(key)` finds the slot for `key`; `.or_default()` says "if that slot
is empty, put a [`Default`](#derive-default) value there first" — for a `BTreeMap` that's
an empty map — and returns a `&mut` to whatever's now in the slot; then `.insert(...)`
adds the field to that record. It's the one-liner for "get this key's record, creating an
empty one on first use."

**Without it** you'd branch by hand:
```rust
if !self.records.contains_key(key) {
    self.records.insert(key.to_owned(), BTreeMap::new());
}
self.records.get_mut(key).unwrap().insert(field, ...);
```
— two lookups and an [`.unwrap()`](#unwrap). `.entry().or_default()` does it in one
lookup and no unwrap. (Its cousin `.or_insert_with(...)` lets you supply a custom
starting value instead of the default.)

First seen in: [In-Memory Database](../patterns/in-memory-database/solution.rs.md)

## closures — `|x| ...` {#closures}

**In one line:** a small anonymous function written inline, often to tell an iterator
adapter *what* to do to each item.

`|expiry| expiry - timestamp` is a function with one parameter `expiry` whose body is
`expiry - timestamp`. The `| |` hold the parameters; what follows is the body. Closures
can **capture** variables from around them — here `timestamp` isn't a parameter, it's
grabbed from the enclosing function — which is exactly why they're handier than a
top-level `fn` for these one-off transforms.

You'll see them destructure tuples too: `|(field, _)| field.starts_with(prefix)` takes
one `(key, value)` item, binds `field` to the first part, and `_` throws away the
second. They feed the [iterator adapters](#iterator-adapters) below and methods like
[`Option::map`](#option).

First seen in: [In-Memory Database](../patterns/in-memory-database/solution.rs.md)

## iterator adapters — `.filter()`, `.map()`, `.collect()`, … {#iterator-adapters}

**In one line:** chainable steps that describe a transformation over a sequence, doing
no work until something asks for the results.

An iterator is a lazy stream of items. **Adapters** reshape the stream; a **consumer**
runs it. Trace the chain in `format_live_fields`:
```rust
fields
    .filter(|(_, stored)| stored.is_alive_at(timestamp))
    .map(|(field, stored)| format!("{field}({})", stored.value))
    .collect()
```
- start: a stream of `(&String, &TimedValue)` pairs.
- `.filter(pred)` keeps only items where the [closure](#closures) returns `true` — here,
  the live ones. Still a stream.
- `.map(f)` transforms each surviving item — here into a `String` like `name(alice)`.
  Still a stream.
- `.collect()` is the **consumer**: it runs the whole chain and gathers the results into
  a collection — a `Vec<String>` here, inferred from the return type.

Two more used in this solution:
- `.filter_map(f)` combines filter + map: the closure returns an [`Option`](#option), and
  `Some(x)` keeps `x` while `None` drops the item. `capture_live_state` uses it to build
  a snapshot key only when its record has live fields.
- `.take_while(pred)` yields items until the first time `pred` is false, then **stops** —
  which is what ends a prefix [range](#btreemap-range) walk at the edge of the block.

**Why chains over loops.** Each adapter is zero-cost (it compiles down to the same loop
you'd write by hand) but reads as a sentence: *filter to live, format each, collect*. And
nothing allocates until `.collect()`.

First seen in: [In-Memory Database](../patterns/in-memory-database/solution.rs.md)

## `impl Trait` in argument position {#impl-trait-arg}

**In one line:** a parameter type that says "any type that can do X," instead of naming
one concrete type.

```rust
fn format_live_fields<'a>(
    fields: impl Iterator<Item = (&'a String, &'a TimedValue)>,
    timestamp: Timestamp,
) -> Vec<String>
```
`fields: impl Iterator<Item = ...>` means "pass me **anything** that iterates over
`(&String, &TimedValue)` pairs." That's what lets **one** function serve both callers:
`scan_at` passes a plain map iterator, and `scan_by_prefix_at` passes a
`range(...).take_while(...)` chain — two *different* concrete iterator types, both
accepted here.

**Why not name the type?** You can't, sanely. The type of a `.range(..).take_while(..)`
chain is a monstrous nested generic the compiler builds internally; writing it out would
be unreadable and would change if you tweaked the chain. `impl Trait` lets you say what
the value *does* (it iterates) rather than *what it is*.

**Why not `&[...]` or `Vec`?** Taking a slice or vector would force each caller to first
*collect* its items into an allocation just to hand them over — defeating the point.
Accepting `impl Iterator` means the items stream straight through with no middle
collection.

(The `<'a>` is a [lifetime](#lifetimes) — see below — tying the borrowed `&String`s in
the items to the same borrow, so the compiler knows how long they live.)

First seen in: [In-Memory Database](../patterns/in-memory-database/solution.rs.md)

## lifetimes — `<'a>` {#lifetimes}

**In one line:** a label the compiler uses to check that a borrow doesn't outlive the
data it points into — you're naming a borrow so its span can be tracked, not creating
anything at runtime.

**The idea first.** Every reference (`&T`) borrows something that lives somewhere. Rust
must be sure the reference is gone before the thing it points to is. Usually it figures
this out silently. Sometimes — when references flow *through* a function's signature in a
non-obvious way — you have to give the borrow a name so the compiler can connect the
dots. That name is a lifetime, written `'a` (an apostrophe plus a word).

**Where it shows up here.**
```rust
fn format_live_fields<'a>(
    fields: impl Iterator<Item = (&'a String, &'a TimedValue)>,
    ...
```
The iterator yields items that *contain borrows* — `&String` and `&TimedValue`. The
`<'a>` declares a lifetime named `a`, and tagging both references with it says "these two
borrows share one lifetime `'a`, and the iterator's items are valid for as long as `'a`
lasts." It ties the borrowed field names and values to the record they came from, so the
compiler can guarantee we don't hold them after that record is gone.

**Without it.** Drop the `<'a>` and write `Item = (&String, &TimedValue)` and the
compiler objects that it can't tell how long those references are meant to live — the
borrows are "unconstrained." Naming the lifetime resolves it. (It's needed here, but not
on simpler borrows like `&self` methods, where Rust infers the lifetime for you — that
inference is called *elision*.)

**Runtime cost:** none. Lifetimes are erased after checking; they exist only to prove the
code is memory-safe at compile time.

First seen in: [In-Memory Database](../patterns/in-memory-database/solution.rs.md)

## `format!` — building strings {#format}

**In one line:** builds a `String` from a template with `{}` holes filled by values.

`format!("{field}({})", stored.value)` produces a string like `name(alice)`. Two filling
styles appear: `{field}` pulls the variable named `field` directly into the hole (a
"captured identifier"), while the empty `{}` is filled by the next argument,
`stored.value`. It's the same templating as `println!`, except `format!` *returns* the
string instead of printing it.

First seen in: [In-Memory Database](../patterns/in-memory-database/solution.rs.md)

## `{:.1}` — format specifiers (decimals, width, …) {#format-spec}

**In one line:** the part after the `:` inside `{}` controls how a value is
*displayed* — `{:.1}` means "one digit after the decimal point" — without changing
the value itself.

A `{}` hole can carry formatting instructions after a colon: `{:.1}`, `{:5}`,
`{:08.2}`, and so on. The most common is **precision** for numbers, written
`.N` — it fixes how many digits show after the decimal point, rounding (and padding
with zeros) to fit:

```rust
let f = 77.0_f32;
println!("{}", f);      // 77      <- plain Display drops the trailing .0
println!("{:.1}", f);   // 77.0    <- forced to one decimal place
println!("{:.2}", 3.14159); // 3.14    <- rounded to two places
```

This is why a Celsius→Fahrenheit result of `77.0` prints as `77` under plain `{}`
but as `77.0` under `{:.1}`. Key distinction: this is about **display only**. The
`f32` in memory is unchanged — `{:.1}` doesn't round the stored value, just the
text. So in `println!("{} Celsius = {:.1} Fahrenheit", celsius, fahrenheit)` the
Celsius side (plain `{}`) shows `25` while the Fahrenheit side shows `77.0`, even
though both are `f32`. If you want *both* shown with a decimal, give both a spec
(`{:.1}`).

Precision is just one specifier — the same slot also does width (`{:5}` → pad to 5
chars), zero-fill (`{:05}`), alignment, sign, and more.

First seen in: [Celsius → Fahrenheit](../challenges/celsius-to-fahrenheit/README.md)

## `.to_owned()` / `.clone()` — making an owned copy {#to-owned-clone}

**In one line:** turn a borrowed view into an owned value you can store, or duplicate an
owned value so you have your own.

Map keys and stored values must be **owned** — a `HashMap<String, ...>` owns its `String`
keys; it can't hold a borrowed `&str` that might vanish. So when we store, we convert:
- `key.to_owned()` turns the borrowed `&str` parameter into an owned `String` to use as a
  key. (`.to_owned()` and `.to_string()` do the same thing for `&str` → `String`;
  `to_owned` is the general "borrowed → owned" name.)
- `stored.value.clone()` duplicates an existing `String` when we need a second owned copy
  (building a snapshot from live data, or rebuilding live data from a snapshot).

Each of these allocates, so they're not free — but storing in an owned collection genuinely
*requires* ownership, so the copies are necessary, not waste. The rule of thumb: borrow
(`&str`, `&T`) in function signatures for cheap reads; reach for `.to_owned()`/`.clone()`
only at the moment you must *keep* the data.

First seen in: [In-Memory Database](../patterns/in-memory-database/solution.rs.md)

## `Copy` types — values duplicated on assignment {#copy}

**In one line:** for small, stack-only types (`i32`, `bool`, `char`, `f64`, …),
`let b = a` makes a second independent copy instead of sharing or moving — and `a`
stays usable.

**What actually happens.** These values live entirely on the stack; the value *is* its
bytes. So `let b = a` just duplicates those bytes into a new box:

```rust
let mut a = 5;
let b = a;   // b is a separate copy
a = 99;      // only a changes; b is still 5
```

The same duplication happens when you pass one into a function — the argument is copied
onto the callee's frame, so the caller's variable is untouched.

**Why this matters / the "without it" contrast.** Not every type is `Copy`. A type that
owns data elsewhere in memory (a `String`, which holds a pointer to heap data) is *not*
`Copy`: for those, `let b = a` **moves** ownership and `a` becomes unusable, because
duplicating the little stack pointer would leave two owners of one heap allocation. So
`Copy` is Rust's marker for "safe and cheap to duplicate byte-for-byte." Knowing which
camp a type is in is the foundation of ownership (moves).

First seen in: [From-Zero Concept 06](../from-zero/rust/06-copy-types/use-it.md)

## `&T` — shared references (borrowing) {#borrow}

**In one line:** `&value` hands out a *reference* — a pointer that lets code read a value
without owning it, so the owner keeps the value and nothing is copied.

**What it's for.** Taking a value by ownership (`s: String`) *moves* it in, so the caller
loses it; cloning copies the whole thing. When a function only needs to *read*, take a
shared reference instead:

```rust
fn length(s: &String) -> usize { s.len() }

let text = String::from("hello");
let n = length(&text);   // borrow, don't move
println!("{text}");      // still usable — text was never given away
```

The `&` appears twice: `&String` in the parameter type ("I take a reference"), and
`&text` at the call ("borrow mine"). See also [`&` in patterns](#ref-pattern), which is
the reverse move — peeling a reference back off in a pattern.

**Two rules that make it safe and cheap.**
- A reference may never **outlive** the value it points at — the compiler rejects any
  reference that could dangle, so a borrow always points at live data. (The mechanism is
  [lifetimes](#lifetimes).)
- A plain `&T` is **read-only** (`shared`). You can hold many `&T` to the same value at
  once, but none can mutate through it. To modify, use [`&mut`](#mut-ref) — the exclusive
  reference — which is the counterpart with its own borrow rules.

A reference is a fixed-size address regardless of how large the borrowed value is, so
borrowing to read is the default in idiomatic Rust; take ownership only when you must
*keep* the value.

First seen in: [From-Zero Concept 10](../from-zero/rust/10-borrowing-with-ref/use-it.md)

## slices — `&str` and `&[T]` {#slice}

**In one line:** a slice is a reference to a *contiguous range* of a value — a pointer to
where the range starts plus a length — so it borrows part of a collection without copying.

**String slices.** `&s[start..end]` on a `String` yields a `&str`, a view into the
existing heap buffer:

```rust
let s = String::from("hello world");
let hello = &s[0..5];   // "hello"  (end is exclusive)
let world = &s[6..11];  // "world"
```

Range shorthands: `[..n]` from the start, `[n..]` to the end, `[..]` the whole thing.

**`&str` vs `String`.** A `String` *owns* growable heap text (ptr + len + capacity); a
`&str` is a *borrowed* two-word view (ptr + len) that owns nothing. A string literal like
`"hello"` is a `&str` pointing at read-only bytes baked into the program — same type as a
slice of a `String`, just pointing at a different place. Prefer `&str` parameters for
read-only text: they accept both literals and slices of a `String`. See
[`String`](#string) and [`&T` borrowing](#borrow).

**They obey the borrow rules.** A slice is a shared borrow, so while one is alive you
can't take a `&mut` to grow the source (`push_str` fails with E0502) — the borrow checker
prevents the slice from being left dangling if the buffer reallocated.

**Array/vec slices.** The same idea gives `&[T]` — a window into an array or `Vec<T>`:
`&numbers[1..3]` is a slice of two elements. Identical shape: pointer + length, no copy.

First seen in: [From-Zero Concept 12](../from-zero/rust/12-slices/use-it.md)

## generics — `<T>` {#generics}

**In one line:** `<T>` is a stand-in for "some type, decided later," so you write a function
or type *once* and use it with every type instead of copy-pasting one version per type.

**On a function.** A `<T>` after the name introduces a type parameter; use `T` where a
concrete type would go. Rust infers what `T` is from how you call it:

```rust
fn first<T>(pair: (T, T)) -> T { pair.0 }

first((10, 20));      // T = i32
first(("hi", "bye")); // T = &str
```

**On a struct.** Types are generic too; multiple parameters (`<T, U>`) let fields differ:

```rust
struct Point<T> { x: T, y: T }        // both fields the same type
struct Pair<T, U> { a: T, b: U }      // two independent types — like HashMap<K, V>
```

**The catch — a bare `T` can only be shuffled, not inspected.** Since the code must work for
*every* type, Rust rejects any operation not guaranteed for all types: `if a > b` on two `T`
won't compile. You can move/return/store a `T`, but to compare, print, or add it you must add
a **trait bound** (`<T: PartialOrd>`) promising what `T` can do. See
[Concept 19](../from-zero/rust/19-generics/use-it.md).

**Zero-cost at runtime.** The compiler *monomorphizes*: it stamps out a separate concrete copy
of the generic code for each type actually used, so a generic call is byte-for-byte as fast as
a hand-written type-specific one. The cost is paid in compile time and binary size, never at
runtime — see [Concept 19 · Under the hood](../from-zero/rust/19-generics/under-the-hood.md).

First seen in: [From-Zero Concept 19](../from-zero/rust/19-generics/use-it.md)
