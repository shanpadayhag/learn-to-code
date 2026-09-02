# Concept 42 · Modules — Under the hood

> Pair: [Use it](use-it.md) · **Under the hood** (you are here)
> Track: [From-Zero: Rust](../README.md)

## The short version
A module is a **compile-time** thing and only a compile-time thing. It has no address, no size, no representation in memory, and nothing happens at run time when you "enter" one. By the time your program starts, the module tree has already done its whole job and been thrown away.

That sentence is doing more work than it looks like, because in most languages it is false. Understanding *why* it is true in Rust explains the two facts that surprise people most: that a `.rs` file nobody declares is not compiled at all, and that `pub` is free.

## What `mod` is not
It is worth naming the two wrong models directly, because almost everyone arrives carrying one of them.

**It is not `#include`.** C's `#include` is textual: the preprocessor pastes another file's contents where the line was, before the compiler sees anything, and the same file pasted twice really is compiled twice. Rust's `mod weather;` inserts nothing. It tells the compiler that a module named `weather` exists in the tree and that its body is stored in another file. Write `mod weather;` twice and you get an error, not two copies.

**It is not Python's `import`.** In Python, `import weather` is a statement that *runs*: it locates a file, executes it top to bottom, builds a module object, and binds it to a name in a dictionary. Afterwards, `weather.report` is a live dictionary lookup, and both the dictionary and the lookup exist while your program runs. In Rust, `weather::report()` is resolved to one specific function while you compile, and the generated instruction is a direct call to a fixed address. There is nothing to look up because the answer was decided long before the program started.

The reason Rust *can* work this way is the compilation unit. **The whole crate compiles as one piece.** When rustc starts on your `main.rs`, it follows every `mod` line, reads every file that any of them names, and treats the result as a single body of code. That is why it can resolve `crate::weather::sensors::read_celsius` to a definite item with no ambiguity and no runtime machinery: it has all of it in front of it at once.

And that is where the surprising file rule comes from. The compiler never scans your folder. It starts at the root file and walks the `mod` lines, so a `scratch.rs` that nothing declares is simply never opened. Not compiled, not type-checked, not warned about. It is not part of your program; it is a file that lives near your program.

## Where the path actually goes
The module path does not vanish entirely — it gets baked into the *name* of the compiled item. Ask a running program what it calls a type and it will tell you the full path:

```rust
mod weather {
    pub mod sensors {
        pub struct Reading { pub celsius: f64 }
    }
}

fn main() {
    println!("{}", std::any::type_name::<weather::sensors::Reading>());
    println!("{}", size_of::<weather::sensors::Reading>());
}
```

```
tn::weather::sensors::Reading
8
```

Two things there. The path survives as a *label* — including the crate name at the front, `tn`, which is what this file happens to be called. And the size is 8, exactly what a bare `struct Reading { celsius: f64 }` at the top of the file would be. Wrapping a type in three modules costs it nothing, because the modules were never part of the value.

You can see the same thing at a lower level. Compile a program with a function nested two modules deep, and look at the symbol names in the binary:

```
__RNvNtNtCs70Xb7SjcONj_6mangle7weather7sensors12read_celsius
```

That is **name mangling**: the path written as length-prefixed segments — `6mangle`, `7weather`, `7sensors`, `12read_celsius` — plus a hash identifying the crate. The linker needs every symbol in the finished binary to have a unique name, and the module path is what makes yours unique. So the tree does leave a trace, in the *names* of things. Never in the running of them.

## What privacy costs, and what it buys
`pub` is a **check**, not a mechanism. The compiler verifies every path you wrote against the visibility rule, and if all of them pass, it emits code that could not tell you what `pub` was. There is no access flag stored anywhere, no check performed on call. Marking something `pub` or leaving it private changes zero bytes and zero cycles in the output.

So privacy is free. But it is not *only* free — it buys something real, and this is the part worth carrying forward.

Go back to the `Gauge` from the [Use it](use-it.md) lesson: `readings` and `total`, both private, with `average` dividing `total` by the count instead of summing. That shortcut is correct *because* the fields are private. There is no reachable line of code anywhere in the program that could have pushed a reading without adding to the total. Privacy turned "the code is careful" into "the compiler can prove no other code exists."

Now notice that this is exactly what every type you have met in Phases 8 through 11 is built on. `Vec` is a pointer, a length and a capacity — and all three are private, which is the only reason `Vec` can promise that the first `len` slots of its buffer are always initialised. `String` is a `Vec<u8>` that promises to hold valid UTF-8; the byte vector is private, so no outside line can put a stray byte in it, which is what makes [Concept 12a](../12a-string-indexing/use-it.md)'s guarantee hold. `Rc` keeps its counts private. `Box` and `NonNull` keep the raw pointer private.

Every one of those is a safe API wrapping something that would be unsound if you could poke at it, and the module wall is *the* thing separating the two. This is the pattern from [Concept 40](../40-unsafe/use-it.md) — a safe surface, a written invariant, audited code underneath — and now you can see the fourth ingredient it needed all along: a wall, so that "audited" means a finite, findable amount of code rather than the whole program.

There is one narrow way privacy touches optimisation directly. Because a private item cannot be named from outside the module and rustc compiles the whole crate at once, it can see *every* call to that item. Nothing else could possibly be calling it. That makes a private function much easier to inline aggressively or delete entirely if unused — the "is it unused?" question is answerable. For a `pub` item in a library, it is not: something you have never seen might call it, so the function has to survive in the compiled artifact.

## Predict the memory
```rust
mod deep {
    pub mod deeper {
        pub mod deepest {
            pub struct Reading { pub celsius: f64 }
            pub fn make() -> Reading { Reading { celsius: 21.5 } }
        }
    }
}

struct Reading { celsius: f64 }

fn main() {
    let nested = deep::deeper::deepest::make();
    let flat = Reading { celsius: 21.5 };

    println!("{} {}", size_of_val(&nested), size_of_val(&flat));
    println!("{}", std::any::type_name::<deep::deeper::deepest::Reading>());
}
```

Before you run it: how many bytes does `nested` occupy, how many does `flat`, and where does `nested` live — stack or heap? Then: what does the second line print, and does anything in the *first* line's answer depend on it?

<details>
<summary>Answer</summary>
<p><code>8 8</code>. Both are one <code>f64</code> and nothing else. <code>nested</code> sits in <code>main</code>'s stack frame exactly like <code>flat</code> does — three levels of module did not add a header, a pointer, a tag, or a hop. There is no such thing as "inside a module" at run time; there is only a value in a slot.</p>
<p>The second line prints <code>&lt;crate&gt;::deep::deeper::deepest::Reading</code>, with the crate name first. That is the <em>only</em> place the nesting still exists: in the name the compiler uses to keep this <code>Reading</code> distinct from the one at the root. And no, the first line's answer does not depend on it at all — the two <code>Reading</code> types are different types with different names and identical layouts, which is the point. The path disambiguates the name; it never touches the bytes.</p>
</details>

## Next
- The track's macro-phases continue with **errors that carry meaning**: [Concept 43 — Custom error types](../43-custom-error-types/use-it.md), where a private field and a `From` impl turn `?` into something that works across your own error types.
