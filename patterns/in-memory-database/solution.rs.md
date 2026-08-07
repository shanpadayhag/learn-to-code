# In-Memory Database — Rust syntax

Notes on the syntax in [`solution.rs`](solution.rs). New features are explained in the
[Rust handbook](../../languages/rust.md); already-known ones are linked there.

## New here
Each entry is a one-sentence "what and why"; the full treatment is one click away in
the handbook.
- `struct TimedValue { ... }` — [`struct`](../../languages/rust.md#struct): defines our
  own type bundling a value with its optional deadline.
- `type Record = BTreeMap<String, TimedValue>;` —
  [`type` alias](../../languages/rust.md#type-alias): a short, intention-revealing name
  for a long nested map type.
- `#[derive(Default)]` / `Self::default()` —
  [derive Default](../../languages/rust.md#derive-default): auto-builds the empty
  starting database, exposed as `new()`.
- `match self.expires_at { ... }` — [`match`](../../languages/rust.md#match): handles
  both the "has a deadline" and "immortal" cases, exhaustively.
- `BTreeMap<K, V>` — [`BTreeMap`](../../languages/rust.md#btreemap): the
  [sorted map](../../glossary/sorted-map.md) that keeps fields (and backups) in order.
- `record.range(prefix.to_owned()..)` / `backups.range(..=t).next_back()` —
  [`.range(...)`](../../languages/rust.md#btreemap-range): walks only a slice of a
  sorted map — the prefix block, or the latest backup at or before a time. (Includes
  the `to_owned()` compile gotcha.)
- `Option<Timestamp>` as a field/return, and `.map(...)` on it —
  [`Option`](../../languages/rust.md#option): a deadline-or-nothing, a value-or-nothing.
- `self.records.get(key)?.get(field)?` —
  [`?`](../../languages/rust.md#question-mark): bail out with `None` the moment a key or
  field is missing.
- `let Some(record) = ... else { return false; };` —
  [`let ... else`](../../languages/rust.md#let-else): unwrap and keep the value flat, or
  leave early.
- `.entry(...).or_default().insert(...)` —
  [`.entry().or_default()`](../../languages/rust.md#entry-or-default): get-or-create a
  record in one lookup.
- `|(field, _)| field.starts_with(prefix)` —
  [closures](../../languages/rust.md#closures): inline anonymous functions that
  destructure tuples and capture surrounding variables.
- `.filter(...)`, `.map(...)`, `.filter_map(...)`, `.take_while(...)`, `.collect()`,
  `.iter()` — [iterator adapters](../../languages/rust.md#iterator-adapters): a lazy,
  chainable description of each traversal.
- `fields: impl Iterator<Item = ...>` —
  [`impl Trait` argument](../../languages/rust.md#impl-trait-arg): one helper that
  accepts either the plain-scan iterator or the prefix-range iterator.
- `fn format_live_fields<'a>(...)` —
  [lifetimes](../../languages/rust.md#lifetimes): names the borrow shared by the
  iterator's `&String`/`&TimedValue` items.
- `format!("{field}({})", stored.value)` —
  [`format!`](../../languages/rust.md#format): builds the `field(value)` output strings.
- `key.to_owned()` / `stored.value.clone()` —
  [`.to_owned()` / `.clone()`](../../languages/rust.md#to-owned-clone): make the owned
  copies a map must hold.

## Already covered
- `use std::collections::{BTreeMap, HashMap};` —
  [`use`](../../languages/rust.md#use) (grouped import of two names).
- `impl InMemoryDatabase { ... }` / `impl TimedValue { ... }` —
  [`impl` block](../../languages/rust.md#impl).
- `pub fn ...` and `fn ...` — [`pub fn`](../../languages/rust.md#pub-fn) (private helpers
  drop the `pub`).
- `HashMap<K, V>` — [`HashMap`](../../languages/rust.md#hashmap).
- `&str`, `&mut self`, `&self` — [`&` / references](../../languages/rust.md#ref-pattern)
  and [`&mut`](../../languages/rust.md#mut-ref).
- `Vec<String>` / `Vec::new()` — [`Vec`](../../languages/rust.md#vec-type).

## Line by line
The interesting methods — the rest follow the same shapes.

**Liveness, in one place.**
```rust
fn is_alive_at(&self, timestamp: Timestamp) -> bool {
    match self.expires_at {
        Some(expiry) => timestamp < expiry,
        None => true,
    }
}
```
The whole TTL rule, named once. [`match`](../../languages/rust.md#match) forces both
cases: a deadline means "before it?", no deadline means "always." Every read, scan, and
backup calls this instead of re-deriving the `<` comparison — change the boundary here
and it changes everywhere.

**The shared write path.**
```rust
self.records.entry(key.to_owned()).or_default().insert(
    field.to_owned(),
    TimedValue { value: value.to_owned(), expires_at },
);
```
[`.entry().or_default()`](../../languages/rust.md#entry-or-default) gets the record for
`key`, creating an empty [`BTreeMap`](../../languages/rust.md#btreemap) on first use, and
`.insert` writes the field. The three [`.to_owned()`](../../languages/rust.md#to-owned-clone)
calls turn the borrowed `&str` parameters into the owned `String`s the maps must hold. The
`expires_at` field uses *field-init shorthand* — a local named `expires_at` fills the field
of the same name.

**A read that can miss twice.**
```rust
let stored = self.records.get(key)?.get(field)?;
if stored.is_alive_at(timestamp) {
    Some(stored.value.as_str())
} else {
    None
}
```
The two [`?`](../../languages/rust.md#question-mark) handle "no such key" and "no such
field" by returning `None` early. Then one liveness check decides between the value (as a
borrowed [`&str`](../../languages/rust.md#string), no copy) and `None` for an expired field.

**Prefix scan — the sorted-map payoff.**
```rust
let fields_with_prefix = record
    .range(prefix.to_owned()..)
    .take_while(|(field, _)| field.starts_with(prefix));
format_live_fields(fields_with_prefix, timestamp);
```
[`.range(...)`](../../languages/rust.md#btreemap-range) seeks to the first field ≥ `prefix`
and `.take_while` stops at the end of the block — together they read only the matching
fields. The bound is `prefix.to_owned()` (not `prefix`) to satisfy the `BTreeMap` key
comparison; the handbook entry explains the `Borrow` error you hit otherwise. The result
feeds the shared formatter via an [`impl Iterator`](../../languages/rust.md#impl-trait-arg)
parameter, so this range chain and the plain `.iter()` in `scan_at` reuse one function.

**Backup: snapshot only what's alive, storing *remaining* time.**
```rust
let remaining_ttl = stored.expires_at.map(|expiry| expiry - timestamp);
```
[`Option::map`](../../languages/rust.md#option) converts an absolute deadline into a
duration (`Some(25)` → `Some(15)`), leaving immortal fields as `None`. The subtraction is
safe because this runs after a `.filter` to live fields, so `timestamp < expiry` — no
[`usize`/`u64` underflow](../../languages/rust.md#usize).

**Restore: find the snapshot, then rebase.**
```rust
let Some((_, snapshot)) = self.backups.range(..=timestamp_to_restore).next_back() else {
    return;
};
self.records = rebuild_records(snapshot, timestamp);
```
[`let ... else`](../../languages/rust.md#let-else) grabs the latest backup at or before the
target (or returns if there is none). `rebuild_record` mirrors backup —
`snapshot_value.remaining_ttl.map(|remaining| timestamp + remaining)` — turning the stored
duration back into an absolute deadline measured from the restore time.
