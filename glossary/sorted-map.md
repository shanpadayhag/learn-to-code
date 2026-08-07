# Sorted Map

**In one line:** a container that stores **key → value** pairs like a
[hash map](hash-map.md), but keeps the keys in **sorted order** at all times — so
it can also answer "everything from here to there" and "the biggest key that's
still ≤ X".

## Plain explanation
A hash map is a coat check: instant for "give me *this exact* ticket's coat," but
the tickets are stored in no particular order, so "give me every coat with a ticket
starting with **A**" means checking the whole rack.

A sorted map is a **library shelf with the books in alphabetical order**. Finding
one exact title is still fast — you don't start at *A* and walk, you jump to about
where it should be and home in (that's a [binary search](big-o-notation.md), roughly
`log n` steps). But because the shelf is *ordered*, two extra questions become easy
that the coat check couldn't answer:

1. **Ranges.** "Every book from *Ma* to *Mz*" is one unbroken stretch of shelf. You
   find where *Ma* starts and just read forward until you pass *Mz*. You never touch
   the *A*s or the *Z*s.
2. **Nearest neighbour.** "The latest edition published *at or before* 1990" — jump
   to 1990 and step back one. No scan.

Under the hood the keys live in a **balanced tree** (or a similar ordered
structure): every insert slots the key into its sorted place and the tree
re-balances so its height stays about `log n`. That height is why *every* operation
— insert, look up, find-the-range-start — costs about `log n` steps instead of the
hash map's ~1.

(Names you'll meet: `BTreeMap` in Rust, `std::map` in C++, `TreeMap` in Java. All
the same idea: an ordered map backed by a balanced tree.)

## Why you care
Reach for a sorted map the moment "in order" or "between" or "closest" enters the
question — the exact places a hash map falls down:

- **Prefix search** ("every field starting with `user_`"): sorting puts all matches
  in one contiguous block, turning the search into a *seek then walk* — `O(log n + m)`
  for `m` matches — instead of scanning all `n` keys and sorting the survivors.
- **"Latest thing at or before time T"**: a backward step from T, `O(log n)`, instead
  of scanning every entry.
- **Always-sorted iteration**: walking the map hands keys back in order for free — no
  separate sort step, ever.

The trade you're making: point lookups become `O(log n)` instead of the hash map's
average `O(1)`. If you *only* ever look up exact keys, use a
[hash map](hash-map.md). The instant you also need order or ranges, the sorted map
pays for that slightly slower lookup many times over.

## Quick examples
Keys `age, name, nickname, note`, asking for prefix `n`:

```
age   name   nickname   note        ← stored in sorted order
      └────── n… block ──────┘
      ^ binary-search to here, then walk right until a key stops starting with "n"
```

You land on `name` in ~`log n` steps and read three neighbours. `age` is never
looked at. A hash map would have scattered these four across random buckets, forcing
you to check all of them and then sort.

A mental picture of the speed: a hash map is a pile of numbered claim tickets — one
exact ticket is instant, but "all tickets in the 400s" means digging through the
whole pile. A sorted map is those same tickets already fanned out in numeric order —
"all tickets in the 400s" is one grab.

## In code
How each language spells this concept:
- Rust: [`BTreeMap<K, V>`](../languages/rust.md#btreemap) and its
  [`.range(...)`](../languages/rust.md#btreemap-range) method

## Related
- [Hash Map](hash-map.md)
- [Big-O notation](big-o-notation.md)

## Shows up in
- [In-Memory Database](../patterns/in-memory-database/README.md)
