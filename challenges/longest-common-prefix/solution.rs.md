# Longest Common Prefix — Rust syntax

Notes on the syntax in [`solution.rs`](solution.rs). Features already in the
[Rust handbook](../../languages/rust.md) are linked; the two that are **new to this repo's
challenges** — the `&[&'a str]` parameter and the `|(a, b)| a == b` closure — get the full
treatment here.

## New here — the two you flagged

### `&[&'a str]` — a borrowed list of borrowed text (with a lifetime name)
`fn longest_common_prefix<'a>(words: &[&'a str]) -> &'a str`. This looks scary but it's three
things you already know, stacked, plus one name tag.

**Trace the types, outside in:**
- `&str` — a [borrowed string slice](../../languages/rust.md#slice): a *view* into text someone
  else owns, not an owned `String`. (Concept 12.)
- `&[T]` — a [slice](../../languages/rust.md#slice): a borrowed view of a run of `T`s. So
  `&[&str]` is "a borrowed list, whose elements are borrowed strings." In `main`,
  `words: Vec<&str>` and `&words` hands this function exactly that view.
- `'a` — a **lifetime** label. It doesn't change *what* the type is; it's a name for "how long
  this borrow stays valid." Writing `&[&'a str] -> &'a str` says: *the `&str` I return borrows
  from the same text as the words you gave me, so it lives exactly as long as they do.*

**Could you do it without the `'a`?** For the *parameter* alone, yes — `&[&str]` elides the
lifetime fine. The name becomes necessary because we **return a borrow**. Try
`fn longest_common_prefix(words: &[&str]) -> &str` and the compiler asks: *the `&str` you're
returning — is it borrowed from `words`, or from something local?* It can't guess, so it
errors. The `'a` answers that question: "from `words`." That's the whole job of the name — it
ties the output borrow to an input borrow so the compiler can prove the returned slice never
outlives the data behind it.

**Why do it this way** instead of returning an owned `String`? Because a slice of the first
word is *already sitting in memory* — returning `&first[..length]` hands back a view of it with
**zero allocation**. `-> String` would copy the prefix into a fresh heap buffer for no reason.

> This is your first real **lifetime**. It gets its own From-Zero lesson next
> (**Concept 25 — Lifetimes**); the handbook stub is [`<'a>`](../../languages/rust.md#lifetimes).
> For now: `'a` is a name tag on a borrow, not a new kind of value.

### `|(a, b)| a == b` — a closure (an inline, unnamed function)
`.take_while(|(a, b)| a == b)`. The `|...| ...` is a **closure**: a small function written right
where it's used, with no name.

**Read the shape:** `|parameters| expression`. The bit between the pipes is the parameter list;
the bit after is the body (and its value is what the closure returns). So `|(a, b)| a == b`
takes one argument, immediately destructures it into `a` and `b`, and returns whether they're
equal.

**Trace what flows in:** `.zip(...)` produces `(u8, u8)` pairs — one byte from each word.
`.take_while` calls our closure on each pair and keeps taking while it returns `true`. The
pattern `(a, b)` splits the pair, so `a` is the first word's byte and `b` is the other's;
`a == b` is `true` while the letters match and `false` at the first difference — which is
exactly where `take_while` stops.

**Could you do it without a closure?** Yes — you could write a named `fn bytes_equal(pair: &(u8, u8)) -> bool { pair.0 == pair.1 }`
and pass `bytes_equal`. The closure just saves you naming and placing a one-line helper far from
where it's used; it reads inline, right at the call. (Closures can also *capture* variables from
around them — none needed here, but that's the superpower a plain `fn` doesn't have.)

> Closures get their own From-Zero lesson (in the upcoming *closures & iterators* phase); the
> handbook entry is [closures — `|x| ...`](../../languages/rust.md#closures). One subtlety worth
> a peek: `take_while` actually hands the closure a *reference* to each item, so `a` and `b` are
> `&u8`; Rust's "match ergonomics" quietly makes `(a, b)` bind through that reference, and
> `&u8 == &u8` compares fine. You don't have to think about it — but that's why no `*` appears.

## Already in the handbook
- [`let … else`](../../languages/rust.md#let-else) — `let Some(first) = words.first() else { return ""; }`:
  bind `first` or bail out early, keeping the happy path flat.
- [iterator adapters](../../languages/rust.md#iterator-adapters) — `.zip()`, `.take()`,
  `.take_while()`, `.count()` build the prefix scan in one forward pass; `.lines()`, `.map()`,
  and `.collect()` read and prepare the input.
- [slices `&s[..n]`](../../languages/rust.md#slice) — `&first[..length]` and `&words[1..]`: a
  borrowed sub-view, no copy.
- [`.bytes()`](../../languages/rust.md#string) vs `.chars()` — `.bytes()` yields raw `u8`s
  (one per ASCII letter, and faster); `.chars()` decodes UTF-8. We use bytes because the input
  is lowercase letters. See the Unicode caveat in the [README](README.md#remember-this).

## Line by line (the interesting bits)
- `stdin.lock().lines()` — read input **one line at a time**, so the program answers as soon as
  the last word is typed. (An earlier version used `read_to_string`, which blocks until stdin
  *closes* — fine when input is piped, but in a real terminal it just waits for `Ctrl-D` and
  looks like it hangs. Reading a fixed `n + 1` lines avoids that.)
- `(0..n).map(|_| lines.next().unwrap().unwrap()).collect()` — pull exactly `n` lines into a
  `Vec<String>` (`owned_words`). Each `.lines()` item is a `Result<String, _>`, hence the two
  `.unwrap()`s: one for "was there a line", one for "did the read succeed".
- `owned_words.iter().map(|word| word.trim()).collect()` — build a `Vec<&str>` of trimmed views
  **borrowing from** `owned_words`. That owned `Vec<String>` stays alive to the end of `main`, so
  those `&str` slices (and the `&'a str` the function returns) remain valid — this is the borrow
  the `'a` in the signature is tracking.
- `for word in &words[1..]` — iterate every word *after* the first; `&words[1..]` is a slice
  skipping index 0, since the first word is our yardstick, not something to compare against
  itself.
- `&first[..length]` — the payoff: a slice of the first word, borrowed, returned as `&'a str`.
