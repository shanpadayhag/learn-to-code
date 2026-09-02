# 1. Two Sum

| | |
|---|---|
| Difficulty | Easy |
| Languages  | Rust |
| Pattern    | [Hash Map](../../glossary/hash-map.md) |
| Time/Space | O(n) / O(n) |
| Source     | [LeetCode](https://leetcode.com/problems/two-sum/) |

## The Problem
You're given a list of whole numbers and a single **target** number. Somewhere in
the list there are exactly two numbers that add up to the target. Your job is to
return the **positions** (indexes) of those two numbers.

What matters:
- There is always **exactly one** answer, so you never have to worry about "no
  match" or "many matches".
- You can't use the same element twice (the two positions must be different).
- The list can be large (up to ~10,000 numbers), and values can be negative.

Tiny example:
```
numbers = [2, 7, 11, 15], target = 9
answer  = [0, 1]          because numbers[0] + numbers[1] = 2 + 7 = 9
```

## Understand It

### In plain words
Someone deals out a row of numbered cards and says: "exactly two of these add up to
6 — which two?"

Two things about what they're asking. First, they don't want the *numbers* back,
they want the *positions* — "the 2nd card and the 4th card", not "2 and 4". Second,
they promise there's exactly one such pair hiding in the row. So the whole task is:
find the two spots whose numbers hit the target, and you can stop the instant you
find them.

### The slow, obvious way
Let's use a real handful: cards `numbers = [3, 2, 4]`, and we want a pair adding to
`target = 6`. The obvious idea is to just try every possible pair and check each
sum:

| Pair checked | Sum | Adds to 6? |
|---|---|---|
| 3 + 2 | 5 | no |
| 3 + 4 | 7 | no |
| 2 + 4 | 6 | ✅ yes — positions 1 and 2 |

Three cards only needed three checks. But look at the *shape* of the work: card 1
gets compared with everyone after it, then card 2 with everyone after it, and so
on. For `n` cards that's roughly `n × n` comparisons. On a list of 10,000 numbers
that's 100 million checks — far too slow. (See
[Big-O notation](../../glossary/big-o-notation.md) for why the `n × n` part is what
hurts.)

### The trick
Here's the shift that breaks the nested loop. **You never have to search for two
numbers at once — only for the one partner of the card already in your hand.**

If you're holding a `4` and the target is `6`, there is exactly one number that
finishes the job: `6 − 4 = 2`. So the vague question "do *any* two of these add to
6?" collapses into a sharp one you ask card by card: *"have I already seen a 2?"*

That reframing is only worth anything if you can answer "have I seen this value
before?" *fast*. If checking meant scanning back over every card you'd already
flipped, each check would cost up to `n` steps and you'd be right back at the `n × n`
brute force — you'd have reorganized the work without removing any of it. The win
comes entirely from making that one question instant.

A [hash map](../../glossary/hash-map.md) is exactly the tool that does it. It stores
`value → position`, and instead of scanning its contents it **hashes the value to
jump straight to the slot where the answer would be** — roughly one step, whether
it holds 3 cards or 30,000. That ~O(1) lookup is the load-bearing part: it's what
turns the inner search into a single glance, and so turns the whole `O(n²)` solution
into one `O(n)` pass. Swap in a slow lookup and the trick evaporates.

So we walk the list a single time, and for each card we:
1. Work out the partner we need (`target − current value`).
2. Glance in the map to see if that partner is already there. If it is, we're done.
3. If not, jot the current card into the map so future cards can find *it*.

### Watch it run
Same cards, `numbers = [3, 2, 4]`, `target = 6`. The map stores `value → index` for
every card we've passed.

| Step | Card (index) | Partner needed (6 − value) | Already in map? | What we do | Map afterwards |
|---|---|---|---|---|---|
| 1 | `3` (index 0) | `3` | no | store `3` | `{3: 0}` |
| 2 | `2` (index 1) | `4` | no | store `2` | `{3: 0, 2: 1}` |
| 3 | `4` (index 2) | `2` | **yes — index 1!** | return `[1, 2]` | — |

Look closely at **step 1**: the card is `3` and the partner we need is *also* `3`.
Doesn't that mean it pairs with itself? No — and this is why the order matters. We
check the map *before* adding the current card, so at that moment the map is still
empty and there's no `3` to match; only afterwards do we file `3` away. That
check-then-store order is exactly what stops a number from being its own partner.

By **step 3** we're holding `4` and need a `2` — and we filed a `2` back at step 2,
so the map answers "yes, index 1" in a single glance.

### The answer
The result is `[1, 2]`, because `numbers[1] + numbers[2] = 2 + 4 = 6`.

Why is this guaranteed correct? Because every card *before* the one in your hand is
already in the map. So if a valid partner exists earlier in the list, you will
always find it the moment you reach the second half of the pair. And since the
problem promises exactly one pair exists, the very first match you hit is *the*
answer.

## The Code
### Rust
```rust
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(input_numbers: Vec<i32>, target_sum: i32) -> Vec<i32> {
        let mut seen_number_index: HashMap<i32, i32> = HashMap::new();

        for (current_number_index, &current_number_value) in input_numbers.iter().enumerate() {
            let needed_number_value = target_sum - current_number_value;
            if let Some(&existing_number_index) = seen_number_index.get(&needed_number_value) {
                return vec![existing_number_index, current_number_index as i32];
            }
            seen_number_index.insert(current_number_value, current_number_index as i32);
        }

        vec![]
    }
}
```

**Time:** O(n) — one pass over the list, and each map lookup/insert is O(1) on
average.   **Space:** O(n) — in the worst case we store almost every number in the
map before finding the pair.
**Run:** `rustc solution.rs && ./solution` — [`solution.rs`](solution.rs) wraps the
block above in the scaffolding LeetCode hides (`struct Solution;`) plus a harness
that asserts every example.
**Syntax notes:** [solution.rs.md](solution.rs.md)

## Remember This
When a problem asks *"is there an element with property X?"* and you're tempted to
write a loop inside a loop, ask whether you can **remember what you've seen** in a
[hash map](../../glossary/hash-map.md) and turn the inner search into an instant
lookup. This "trade memory for speed, look back instead of searching forward"
pattern turns `O(n²)` into `O(n)` and shows up everywhere — pair-sum problems,
finding duplicates, substring problems, and counting.
