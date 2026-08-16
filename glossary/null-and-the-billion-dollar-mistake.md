# Null (and the Billion-Dollar Mistake)

**In one line:** `null` is a special "there's nothing here" value that most languages let
you put where a real value should be — and because it hides in plain sight, it has caused so
many crashes that its own inventor called it his "billion-dollar mistake."

## Plain explanation
Imagine every box in a warehouse is supposed to contain a product. To handle the case of "we
don't have that product right now," someone invents a rule: an **empty box** with a special
`NOTHING` sticker counts as a valid box too. You can hand a `NOTHING` box to anyone,
anywhere a box is expected.

That's `null`. A variable that's supposed to hold a name, a number, or a user can instead
hold `null` — meaning "nothing." The catch: from the outside, a `NOTHING` box looks like
every other box. You can't tell by looking.

So a worker grabs a box, opens it to use the product inside — and it's empty. The whole line
stops. In a program, "opening an empty box" is trying to *use* a value that's actually
`null`, and it typically **crashes the program on the spot** (you'll see errors like
`NullPointerException`, "null is not an object", or a segfault).

The deep problem isn't that "nothing" needs representing — it's that with `null`, **the
type system doesn't warn you**. A box labeled "contains a name" might secretly be empty, and
nothing forces you to check before you open it. The danger is invisible until it detonates,
usually at runtime, in front of a user.

## Why it's called the billion-dollar mistake
Tony Hoare introduced the null reference in 1965 because it was easy to implement. Decades
later he apologized for it publicly, estimating the crashes, vulnerabilities, and damage it
caused across the industry at somewhere around a billion dollars. Almost every mainstream
language inherited the idea — and inherited the bug class with it.

## The fix: make "missing" a visible, separate thing
The modern answer is to *not* let "nothing" masquerade as a normal value. Instead, "maybe
missing" gets its **own type** that the compiler can see, so it can force you to handle the
empty case *before* you use the value. You can't accidentally skip the check, because the
code won't compile until you do.

Different languages spell this differently, but it's the same idea:
- **Rust** — [`Option<T>`](../from-zero/rust/15-option/use-it.md): a value is either
  `Some(value)` or `None`, and you can't touch the inside without opening it.
- **Haskell / Scala** — `Maybe` / `Option`.
- **Swift** — *optionals* (`String?`), which must be unwrapped before use.
- **Kotlin / TypeScript** — nullable types (`String?`) that the compiler tracks and refuses
  to let you use unchecked.

The common thread: "nothing" is turned from an invisible trapdoor under *every* value into
one honest, checkable case.

## Quick examples
- **The trap (null):** a `find_user` returns a user *or* `null`. You write
  `user.name` — compiles fine, looks correct, and crashes the day someone isn't found.
- **The safe version (Option):** `find_user` returns `Option<User>`. Now `user.name` won't
  even compile — you're made to handle the "not found" case first, so the crash can't happen.

## Related
- [`Option` — From-Zero Concept 15](../from-zero/rust/15-option/use-it.md) — Rust's
  null-free replacement, built as a plain enum.

## Shows up in
- [From-Zero Concept 15 — `Option` (no more null)](../from-zero/rust/15-option/use-it.md)
