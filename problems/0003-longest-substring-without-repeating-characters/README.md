# 3. Longest Substring Without Repeating Characters

| | |
|---|---|
| Difficulty | Medium |
| Languages  | Rust |
| Pattern    | [Sliding Window](../../glossary/sliding-window.md) |
| Time/Space | O(n) / O(min(n, k)) |
| Source     | [LeetCode](https://leetcode.com/problems/longest-substring-without-repeating-characters/) |

## The Problem
You're given a string — just a row of characters (letters, digits, symbols,
spaces). A **substring** is any unbroken run of characters cut out of it: for
`"abcabcbb"`, `"abc"` and `"bca"` are substrings, but `"acb"` is not (its
characters aren't next to each other in the original). Your job is to find the
**length** of the longest substring in which no character repeats.

What matters:
- You return the **length** (a number), not the substring itself.
- "No repeats" means every character in that run is different from all the others
  in the same run.
- The string can be empty (answer `0`) and can be fairly long (up to ~50,000
  characters), so an approach that re-scans it many times will be too slow.

Tiny example:
```
s = "abcabcbb"
answer = 3        because the longest repeat-free run is "abc" (length 3)
```

## Understand It

### In plain words
Imagine a long strip of coloured tiles laid end to end. You want the **longest
unbroken stretch of tiles where no colour appears twice**. The moment a stretch
would have to include a repeated colour, that stretch has to end. You slide your
eyes along the strip looking for the widest clean run.

Two things to hold onto. First, the run has to be *contiguous* — a stretch of
neighbours, not a hand-picked selection. Second, you only care *how wide* the best
run is, not which tiles it was.

### The slow, obvious way
Let's use a short strip: `s = "abba"`. The obvious idea is to try **every possible
starting tile**, walk forward from it as far as we can without hitting a colour
we've already used in *this* run, and remember the longest run we managed.

| Start at | Walk forward, stop at first repeat | Length |
|---|---|---|
| `a` (index 0) | `a`, then `ab`, then `abb` → the second `b` repeats, stop | 2 (`ab`) |
| `b` (index 1) | `b`, then `bb` → repeats, stop | 1 (`b`) |
| `b` (index 2) | `b`, then `ba` | 2 (`ba`) |
| `a` (index 3) | `a` | 1 (`a`) |

Best run: length **2**. Four tiles only needed a handful of steps here. But look at
the *shape* of the work: from every single start we walk forward and re-inspect
characters we already looked at from an earlier start. On a strip of `n` tiles
that's roughly `n` starts × `n` steps each ≈ `n × n` work. On 50,000 characters
that's billions of steps — far too slow. (See
[Big-O notation](../../glossary/big-o-notation.md) for why the `n × n` part is what
hurts.)

### The trick
Here's the shift. Instead of restarting from scratch at every tile, keep **one
window** — a left edge and a right edge — that always holds a repeat-free run. Push
the **right edge forward one tile at a time**. As long as the new tile's colour
isn't already inside the window, the window just grows. The instant the new colour
*is* already inside, **jump the left edge forward** to just past that earlier copy,
which throws out exactly the tiles causing the clash — and now the window is clean
again.

This is the [sliding window](../../glossary/sliding-window.md) pattern: the two
edges only ever move **forward**, so across the whole run each edge travels the
length of the string once. That's what turns `n × n` into a single `n`-length
pass.

But there's a catch that decides the whole thing: when the right edge lands on a
colour, you have to know **whether it's already in the current window, and where**,
*instantly*. If answering that meant scanning back across the window every step,
each step would cost up to `n` and you'd be right back at the `n × n` brute force —
the window would have reorganized the work without removing any of it.

So we lean on a [hash map](../../glossary/hash-map.md) that remembers, for every
colour, the **last position we saw it**. It stores `character → most recent index`,
and instead of scanning it **hashes the character to jump straight to its slot** —
about one step, no matter how big the string. That ~O(1) lookup is the load-bearing
part: it's what lets each new tile ask "have I seen you, and where?" in a single
glance, and so it's what buys the whole `O(n)` solution. Swap in a slow lookup and
the trick collapses.

One subtlety the map forces us to handle: the last place we saw a colour might be
*before* the left edge — already thrown out of the window. That's not a real
repeat, so we must **only jump the left edge forward, never backward**. We'll see
exactly this in a moment.

So for each tile at the right edge we:
1. Look up whether this colour was seen before, and where.
2. If that earlier spot is **inside the current window** (at or after the left
   edge), move the left edge to just past it.
3. Record this colour's new latest position in the map.
4. Measure the current window's width and keep the largest we've seen.

### Watch it run
Same strip, `s = "abba"` (positions `a`=0, `b`=1, `b`=2, `a`=3). The map stores
`character → last index seen`. `left` is shorthand for the window's left edge —
`current_substring_start_index` in the code below; the window is everything from
`left` to the current position.

| Step | Right edge (index) | Seen this colour before? | Move left edge? | Window | Width | Longest so far | Map afterwards |
|---|---|---|---|---|---|---|---|
| 1 | `a` (0) | no | left stays `0` | `a` | 1 | 1 | `{a: 0}` |
| 2 | `b` (1) | no | left stays `0` | `ab` | 2 | 2 | `{a: 0, b: 1}` |
| 3 | `b` (2) | yes, at index `1` — inside window (`1 ≥ 0`) | jump left to `2` | `b` | 1 | 2 | `{a: 0, b: 2}` |
| 4 | `a` (3) | yes, at index `0` — **outside** window (`0 < 2`) | left stays `2` | `ba` | 2 | 2 | `{a: 2, b: 2}` |

**Step 3** is the ordinary case: the incoming `b` was last seen at index 1, which
is inside our current window, so it's a genuine repeat — we yank the left edge to
index 2, dropping the old `b` and leaving a clean `"b"`.

**Step 4** is the subtle one, and the reason for the "inside the window?" test. The
incoming `a` *was* seen before — at index 0 — but the window now starts at index 2,
so that old `a` was already thrown out. It's **not** a repeat within the current
run. If we'd blindly jumped the left edge to "just past index 0" (i.e. index 1) we'd
have moved it *backward* and corrupted the window. Checking `0 ≥ left` first (it
isn't) is what keeps us honest: we leave the left edge alone and the window grows to
`"ba"`.

### Watch it run on a messier string
`"abba"` was short. Let's do a longer, trickier one — `s = "tmmzuxt"` — because it
shows both cases *and* a case where the window keeps growing for a long stretch
before the twist at the very end. Positions: `t`=0, `m`=1, `m`=2, `z`=3, `u`=4,
`x`=5, `t`=6.

| Step | Right edge (index) | Seen this character before? | Move left edge? | Window | Width | Longest so far | Map afterwards |
|---|---|---|---|---|---|---|---|
| 1 | `t` (0) | no | left stays `0` | `t` | 1 | 1 | `{t: 0}` |
| 2 | `m` (1) | no | left stays `0` | `tm` | 2 | 2 | `{t: 0, m: 1}` |
| 3 | `m` (2) | yes, at index `1` — inside window (`1 ≥ 0`) | jump left to `2` | `m` | 1 | 2 | `{t: 0, m: 2}` |
| 4 | `z` (3) | no | left stays `2` | `mz` | 2 | 2 | `{t: 0, m: 2, z: 3}` |
| 5 | `u` (4) | no | left stays `2` | `mzu` | 3 | 3 | `{t: 0, m: 2, z: 3, u: 4}` |
| 6 | `x` (5) | no | left stays `2` | `mzux` | 4 | 4 | `{t: 0, m: 2, z: 3, u: 4, x: 5}` |
| 7 | `t` (6) | yes, at index `0` — **outside** window (`0 < 2`) | left stays `2` | `mzuxt` | 5 | 5 | `{t: 6, m: 2, z: 3, u: 4, x: 5}` |

Walk the story it tells:

- **Step 3 — the genuine repeat.** The second `m` clashes with the `m` still sitting
  in the window at index 1, so we haul the left edge to index 2, throwing out the old
  `m` (and the `t` before it). The window resets to a clean `"m"`.
- **Steps 4–6 — free growth.** `z`, `u`, `x` are all new, so the right edge just
  keeps marching and the window fattens up: `mz`, `mzu`, `mzux`. The best width climbs
  to 4 without the left edge moving at all.
- **Step 7 — the twist, and it's the one that wins.** The final `t` *was* seen — way
  back at index 0. But the window now starts at index 2, so that old `t` is long gone,
  outside the window. The `0 ≥ left` test fails (`0 < 2`), so we **don't** move the
  left edge, and the window grows to `"mzuxt"` — width **5**. Had we naively jumped the
  edge on "seen before," we'd have wrongly shrunk the window and *missed the real
  answer*.

Notice too that the map's entry for `t` quietly updates from `0` to `6` in the last
step: we always record a character's *most recent* position, so future clashes are
measured against where we last saw it, not where we first did.

### The answer
For `"abba"` the widest window was **2** (`"ab"` or `"ba"`); for `"tmmzuxt"` it was
**5** (`"mzuxt"`). In each case the answer is simply the largest width we ever
recorded.

Why is this guaranteed correct? Two facts. **The window is always repeat-free** —
every time a new character would clash with one still inside, we immediately move
the left edge past the clash, restoring the "all unique" property before we measure.
And **we measure the window at every right-edge position**, so the single widest
repeat-free run the string contains is one of the widths we compared — and we kept
the largest.

## The Code
### Rust
```rust
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(input_text: String) -> i32 {
        let mut last_seen_character_index: HashMap<char, usize> = HashMap::new();
        let mut current_substring_start_index = 0;
        let mut longest_substring_length = 0;

        for (current_character_index, current_character) in input_text.chars().enumerate() {
            if let Some(&existing_character_index) =
                last_seen_character_index.get(&current_character)
            {
                if existing_character_index >= current_substring_start_index {
                    current_substring_start_index = existing_character_index + 1;
                }
            }
            last_seen_character_index.insert(current_character, current_character_index);

            let current_substring_length =
                current_character_index - current_substring_start_index + 1;
            longest_substring_length = longest_substring_length.max(current_substring_length);
        }

        longest_substring_length as i32
    }
}
```

**Time:** O(n) — the right edge visits each character once, and the left edge only
ever moves forward, so together they do at most `2n` steps; every map
lookup/insert is O(1) on average.   **Space:** O(min(n, k)) — the map holds at most
one entry per *distinct* character, so it's capped by both the string length `n`
and the size `k` of the character set (e.g. 128 for ASCII).
**Run:** `rustc solution.rs && ./solution` — [`solution.rs`](solution.rs) wraps the
block above in the scaffolding LeetCode hides (`struct Solution;`) plus a harness
that asserts every example.
**Syntax notes:** [solution.rs.md](solution.rs.md)

## Remember This
When a problem asks for the longest/shortest/best **contiguous run** that satisfies
some rule, reach for the [sliding window](../../glossary/sliding-window.md): keep a
left and right edge, push the right edge forward, and only pull the left edge
forward when the rule is violated — never restart from scratch. Pair it with a
[hash map](../../glossary/hash-map.md) whenever "is this already in my window, and
where?" needs to be answered instantly. Both edges moving forward only is what keeps
it a single `O(n)` pass instead of `O(n²)`.
