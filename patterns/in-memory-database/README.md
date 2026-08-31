# In-Memory Database

| | |
|---|---|
| Difficulty | Hard (multi-level) |
| Languages  | Rust |
| Pattern    | [Sorted Map](../../glossary/sorted-map.md) · [Lazy Expiration](../../glossary/lazy-expiration.md) |
| Time/Space | point ops O(log F) · prefix scan O(log F + M) / O(K·F) |
| Source     | Common industry-coding interview pattern (in-memory key–value store). Paraphrased in full; no public LeetCode link. |

## The Problem
Build a tiny database that lives entirely in memory. Data is addressed by **two
coordinates**: a `key` (think: a row) and a `field` (think: a column). Every key
holds its own set of fields, and different keys can have completely different
fields. The task is delivered in four escalating levels, each adding operations:

**Level 1 — basic records**
- `set(key, field, value)` — store a value at `key`/`field`.
- `get(key, field)` — read it back, or nothing if it's absent.
- `delete(key, field)` — remove it; report whether anything was there.

**Level 2 — scanning a record**
- `scan(key)` — return every `field(value)` under `key`, **sorted by field**.
- `scan_by_prefix(key, prefix)` — same, but only fields whose name starts with
  `prefix`.

**Level 3 — timestamps and expiry** (every operation now carries a `timestamp`)
- `set_at(key, field, value, timestamp)` — a value with no expiry.
- `set_at_with_ttl(key, field, value, timestamp, ttl)` — the value is alive from
  `timestamp` (inclusive) until `timestamp + ttl` (exclusive), then vanishes.
- `get_at`, `delete_at`, `scan_at`, `scan_by_prefix_at` — the Level 1–2 operations,
  each answered *as of* the given timestamp.

**Level 4 — backup and restore**
- `backup(timestamp)` — snapshot the whole database at `timestamp`; return how many
  keys still hold at least one live field.
- `restore(timestamp, timestamp_to_restore)` — replace the current state with the
  most recent backup taken at or before `timestamp_to_restore`, with every TTL
  **re-based** so each field keeps the lifespan it had left.

One constraint quietly shapes everything: **timestamps only ever move forward**.
Operations arrive in strictly increasing time order, so no read ever asks about the
past. Remember that — it's the reason a whole layer of machinery turns out to be
unnecessary.

Tiny example:
```
set_at("user1", "name", "alice", 1)
set_at_with_ttl("user1", "session", "xyz", 5, 10)   // dies at 15
get_at("user1", "session", 12)  ->  "xyz"           // 12 < 15, alive
get_at("user1", "session", 15)  ->  (nothing)       // 15 is the deadline, gone
scan("user1", 16)               ->  ["name(alice)"] // session has expired away
```

## Understand It

### In plain words
Picture a **filing cabinet**. Each **key** is a labelled drawer. Inside a drawer are
**folders** (fields), each holding one slip of paper (the value). The four levels are
four things people ask you to do at this cabinet:

1. File, read, and shred individual slips.
2. Read a whole drawer — or just the folders whose labels start with some letters.
3. Some slips are written in **disappearing ink** with a deadline; every request now
   also tells you *what time it is*.
4. **Photograph** the whole cabinet, and later put it **back** to match a photo — and
   the disappearing-ink slips have to come back with the right amount of ink left.

The skeleton never changes: `key → field → value`. Each level only enriches what
sits in the value slot, or adds a shelf of photos beside the cabinet.

---

### Level 1–2: the shape, and the sorted-map trick

**The slow, obvious way.** Store each drawer's folders in whatever order they were
added. `set`/`get`/`delete` on one folder is fine. But `scan_by_prefix(key, "n")`
now means **check every folder in the drawer** and keep the ones starting with `n` —
and if the folders are in random order, **sort** the survivors before handing them
back. On a drawer of 10,000 folders where 3 start with `n`, you still paw through all
10,000. That's `O(F)` per prefix scan, plus a sort.

**The trick.** Keep every drawer's folders **filed in alphabetical order,
permanently**. That's a [sorted map](../../glossary/sorted-map.md) (in Rust, a
[`BTreeMap`](../../languages/rust.md#btreemap)). Two payoffs fall straight out:

1. **`scan` is pre-sorted** — walking the drawer hands folders back in order, no sort
   step ever.
2. **`scan_by_prefix` becomes a range, not a search.** Every folder starting with `n`
   sits in one unbroken block (sorting put `name`, `nickname`, `note` next to each
   other). So you [binary-search](../../glossary/big-o-notation.md) to the *first*
   folder ≥ `"n"`, then walk straight ahead, stopping the instant a label stops
   starting with `n`. You touch the matches and **nothing else** — `O(log F + M)` for
   `M` matches.

This is the load-bearing claim, so be precise about *why* it's fast: the range walk
is `O(log F)` to find the start (the tree is height-balanced) plus `O(M)` to read the
matches. If the drawer used an unordered [hash map](../../glossary/hash-map.md)
instead, hashing would scatter `name` and `note` into random buckets — there'd be no
"first folder ≥ n" to jump to, so you'd be back to scanning all `F` and sorting. **The
sorted structure is what buys the fast prefix scan.** (The cost you accept: point
lookups become `O(log F)` rather than a hash map's average `O(1)` — worth it, because
prefix scan is a required feature and wants sorted output anyway.)

The two coordinates also tell you the **outer** container: every operation names one
key and never queries across keys, so the outer map only needs instant exact lookup —
a plain hash map. Ordered outer, unordered outer — it'd never matter, so pick the
cheaper `O(1)`.

**Watch it run.** Drawer `"user1"`, stored already-sorted in the `BTreeMap`:

| field | value |
|---|---|
| `age` | `30` |
| `name` | `alice` |
| `nickname` | `al` |
| `note` | `vip` |

`scan_by_prefix("user1", "n")`:

| step | where we are | starts with `"n"`? | action |
|---|---|---|---|
| seek | jump to first field ≥ `"n"` → `name` | — | `O(log F)` landing |
| 1 | `name` | yes | keep `name(alice)` |
| 2 | `nickname` | yes | keep `nickname(al)` |
| 3 | `note` | yes | keep `note(vip)` |
| 4 | (past the end) | — | stop |

Result: `["name(alice)", "nickname(al)", "note(vip)"]` — sorted, and `age` was never
looked at.

---

### Level 3: timestamps and TTL (lazy expiration)

Now the value slot grows a **lifespan**. Every request comes with "…and what time is
it?" attached.

**The slow, obvious way.** Two tempting bits of over-engineering:

- Keep a **full logbook per field** — every value it ever held, tagged with its write
  time — and search the logbook for "the value as of time T." That's `O(V)` memory per
  field for versions you'll almost never read.
- Run a **janitor** every tick that walks all `K·F` fields and shreds expired ones —
  `O(K·F)` work per tick, whether or not anything actually expired.

Both are mostly wasted. Here's why.

**The trick — two ideas.**

1. **Reads only move forward, so you don't need history.** Operations arrive in
   strictly increasing timestamp order, so a `get_at(T)` never asks about a value
   that's already been overwritten. If you can never query the past, there's **no
   reason to remember old versions** — one value per field is enough. *(Say this
   assumption out loud in an interview; it's exactly the reasoning being probed.)*
2. **[Lazy expiration](../../glossary/lazy-expiration.md).** Don't run a janitor.
   Just **record the death time** and check it *at read time*: a field is alive at `T`
   iff it has no expiry, or `T` is still before it. Expired fields cost nothing until
   someone happens to read one.

> **The cousin where idea 1 flips:** *LeetCode 981, Time Based Key-Value Store*, lets
> reads target arbitrary **past** timestamps. There you genuinely need the sorted
> logbook and a binary search over it — `O(log V)` per read. Same shape, opposite
> conclusion. Knowing *why* they differ — history is worth its cost only when reads
> can travel backwards — is the real lesson.

**Watch it run.** Record for `"s"`: `role` set normally at t=1 (immortal); `session`
set with ttl 10 at t=5 (dies at 15). No cleanup is ever run:

| field | value | `expires_at` |
|---|---|---|
| `role` | `admin` | none (forever) |
| `session` | `xyz` | 15 |

| call | field | check | result |
|---|---|---|---|
| `get_at("s","session",12)` | `session` | `12 < 15`? yes | `"xyz"` |
| `get_at("s","session",15)` | `session` | `15 < 15`? no | nothing (faded) |
| `get_at("s","role",99)` | `role` | no expiry | `"admin"` |

`session` "expired" at 15, but nothing touched it to make that happen — the read at 15
simply *computed* that it was past its deadline.

---

### Level 4: backup & restore (TTL re-basing)

`backup` photographs the cabinet. `restore` puts it back to match a chosen photo. The
whole difficulty is one twist with the disappearing-ink slips.

**The slow, obvious way.** Copy each field's **absolute** death time straight into the
photo. Feels natural — just save what's there. Now restore that photo much later:

- `token` expires at **25**. You back up at t=10, copying `expires_at = 25`.
- You restore at t=**100**. `token` comes back still claiming `expires_at = 25`.
- Every read after t=25 — which is *all* of them now — sees `100 < 25`? No. **Dead on
  arrival.** Every TTL'd field silently vanishes the instant you restore into the
  future.

**The trick — two ideas.**

1. **Snapshots store *remaining* time, not absolute deadlines.** At backup time `b`, a
   live field with deadline `e` has `remaining = e − b`. On restore at time `r`, you
   **re-base**: `new_expiry = r + remaining`. The field gets exactly the lifespan it
   had left, measured from the new "now." Immortal fields stay immortal. *(The unsigned
   subtraction `e − b` can't underflow — but only because it runs **after** filtering
   to live fields, where `b < e` is guaranteed. The liveness filter is what makes the
   arithmetic safe.)*
2. **Find the right photo with a range query.** Keep backups in a
   [sorted map](../../glossary/sorted-map.md) keyed by time. "Latest backup at or
   before T" is a backward step from T — `O(log B)`, not a scan of every backup ever
   taken.

**Watch it run.** State at t=10 — `token` was set at t=5 with ttl 20 (dies at 25);
`role` set normally:

| field | value | `expires_at` | alive at 10? | remaining stored |
|---|---|---|---|---|
| `role` | `admin` | none | yes | none |
| `token` | `xyz` | 25 | `10 < 25` ✓ | `25 − 10 = 15` |

`backup(10)` stores `{ "s": {role: (admin, ∞), token: (xyz, 15)} }` and returns **1**
(one key with live fields). Time passes, the DB gets clobbered. Then `restore(100, 10)`:

| step | action |
|---|---|
| find | latest backup at or before 10 → the t=10 snapshot |
| rebuild `role` | remaining ∞ → `expires_at = none` |
| rebuild `token` | remaining 15 → `expires_at = 100 + 15 = 115` |

`get_at("s","token",114)` → `114 < 115` ✓ → `"xyz"`. `get_at("s","token",115)` → gone.
The token came back with its **15 units of life intact**, now measured from t=100 —
exactly the disappearing-ink rule.

### The answer
The `key → field → value` skeleton stays fixed; each level only changes what fills
the value slot or adds a shelf beside it. A **sorted inner map** makes scans sorted
and prefix scans a seek-then-walk. **Lazy expiration** plus the forward-only reads
mean one value per field with a death-stamp is enough, and expiry costs nothing until
read. **Backups store remaining lifespan, not absolute deadlines**, so restore can
re-base every TTL to the moment of restore — and a **sorted shelf of backups** makes
"which photo?" an `O(log B)` seek.

## The Code
### Rust
```rust
use std::collections::{BTreeMap, HashMap};

type Timestamp = u64;

struct TimedValue {
    value: String,
    expires_at: Option<Timestamp>,
}

impl TimedValue {
    fn is_alive_at(&self, timestamp: Timestamp) -> bool {
        match self.expires_at {
            Some(expiry) => timestamp < expiry,
            None => true,
        }
    }
}

struct SnapshotValue {
    value: String,
    remaining_ttl: Option<Timestamp>,
}

type Record = BTreeMap<String, TimedValue>;
type SnapshotRecord = BTreeMap<String, SnapshotValue>;
type Snapshot = HashMap<String, SnapshotRecord>;

#[derive(Default)]
pub struct InMemoryDatabase {
    records: HashMap<String, Record>,
    backups: BTreeMap<Timestamp, Snapshot>,
}

impl InMemoryDatabase {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_at(&mut self, key: &str, field: &str, value: &str, _timestamp: Timestamp) {
        self.store(key, field, value, None);
    }

    pub fn set_at_with_ttl(
        &mut self,
        key: &str,
        field: &str,
        value: &str,
        timestamp: Timestamp,
        ttl: Timestamp,
    ) {
        self.store(key, field, value, Some(timestamp + ttl));
    }

    fn store(&mut self, key: &str, field: &str, value: &str, expires_at: Option<Timestamp>) {
        self.records.entry(key.to_owned()).or_default().insert(
            field.to_owned(),
            TimedValue { value: value.to_owned(), expires_at },
        );
    }

    pub fn get_at(&self, key: &str, field: &str, timestamp: Timestamp) -> Option<&str> {
        let stored = self.records.get(key)?.get(field)?;
        if stored.is_alive_at(timestamp) {
            Some(stored.value.as_str())
        } else {
            None
        }
    }

    pub fn delete_at(&mut self, key: &str, field: &str, timestamp: Timestamp) -> bool {
        let Some(record) = self.records.get_mut(key) else {
            return false;
        };
        let Some(stored) = record.get(field) else {
            return false;
        };
        if !stored.is_alive_at(timestamp) {
            return false;
        }
        record.remove(field);
        true
    }

    pub fn scan_at(&self, key: &str, timestamp: Timestamp) -> Vec<String> {
        match self.records.get(key) {
            Some(record) => format_live_fields(record.iter(), timestamp),
            None => Vec::new(),
        }
    }

    pub fn scan_by_prefix_at(&self, key: &str, prefix: &str, timestamp: Timestamp) -> Vec<String> {
        let Some(record) = self.records.get(key) else {
            return Vec::new();
        };
        let fields_with_prefix = record
            .range(prefix.to_owned()..)
            .take_while(|(field, _)| field.starts_with(prefix));
        format_live_fields(fields_with_prefix, timestamp)
    }

    pub fn backup(&mut self, timestamp: Timestamp) -> usize {
        let snapshot = self.capture_live_state(timestamp);
        let saved_record_count = snapshot.len();
        self.backups.insert(timestamp, snapshot);
        saved_record_count
    }

    pub fn restore(&mut self, timestamp: Timestamp, timestamp_to_restore: Timestamp) {
        let Some((_, snapshot)) = self.backups.range(..=timestamp_to_restore).next_back() else {
            return;
        };
        self.records = rebuild_records(snapshot, timestamp);
    }

    fn capture_live_state(&self, timestamp: Timestamp) -> Snapshot {
        self.records
            .iter()
            .filter_map(|(key, record)| {
                let live_fields = capture_live_fields(record, timestamp);
                if live_fields.is_empty() {
                    None
                } else {
                    Some((key.clone(), live_fields))
                }
            })
            .collect()
    }
}

fn format_live_fields<'a>(
    fields: impl Iterator<Item = (&'a String, &'a TimedValue)>,
    timestamp: Timestamp,
) -> Vec<String> {
    fields
        .filter(|(_, stored)| stored.is_alive_at(timestamp))
        .map(|(field, stored)| format!("{field}({})", stored.value))
        .collect()
}

fn capture_live_fields(record: &Record, timestamp: Timestamp) -> SnapshotRecord {
    record
        .iter()
        .filter(|(_, stored)| stored.is_alive_at(timestamp))
        .map(|(field, stored)| {
            let remaining_ttl = stored.expires_at.map(|expiry| expiry - timestamp);
            (field.clone(), SnapshotValue { value: stored.value.clone(), remaining_ttl })
        })
        .collect()
}

fn rebuild_records(snapshot: &Snapshot, timestamp: Timestamp) -> HashMap<String, Record> {
    snapshot
        .iter()
        .map(|(key, snapshot_record)| (key.clone(), rebuild_record(snapshot_record, timestamp)))
        .collect()
}

fn rebuild_record(snapshot_record: &SnapshotRecord, timestamp: Timestamp) -> Record {
    snapshot_record
        .iter()
        .map(|(field, snapshot_value)| {
            let expires_at = snapshot_value.remaining_ttl.map(|remaining| timestamp + remaining);
            (field.clone(), TimedValue { value: snapshot_value.value.clone(), expires_at })
        })
        .collect()
}
```

**Time:** point ops (`set`/`get`/`delete`) are **O(log F)** in a record of `F` fields
(the outer key hash is `O(1)`); the liveness check is `O(1)`. `scan_at` is **O(F)**;
`scan_by_prefix_at` is **O(log F + M)** for `M` matches. `backup` is **O(N)** over `N`
live fields plus **O(log B)** to file it; `restore` is **O(log B)** to find the
snapshot plus **O(S)** to rebuild `S` fields.
**Space:** **O(K·F)** for live data. Backups are **full copies**, so the shelf costs
**O(B·N)** — flag that tradeoff out loud (deltas or structural sharing if it mattered).
Expired-but-unread fields also linger until overwritten (the lazy-expiry cost).
**Run:** `rustc solution.rs && ./solution` — [`solution.rs`](solution.rs) adds a
harness that drives one scripted session through all four levels and asserts the
state after every step.
**Syntax notes:** [solution.rs.md](solution.rs.md)

## Remember This
Five reusable moves live in this one problem:

1. **Two coordinates → a map of maps.** When data is addressed by `(a, b)`, nest two
   maps — and pick each level's type from what it's asked to do.
2. **Need order, ranges, or prefixes? Use a [sorted map](../../glossary/sorted-map.md).**
   Sorting turns "everything matching a prefix" into one contiguous seek-then-walk.
   A [hash map](../../glossary/hash-map.md) can't do that.
3. **[Lazy expiration](../../glossary/lazy-expiration.md):** don't police deadlines on
   a timer — record the death time and check it on read.
4. **Forward-only reads kill the need for history.** If nothing can query the past,
   store one value, not a version log. (When reads *can* go back — LeetCode 981 — you
   need the log and a binary search.)
5. **Snapshot *relative* lifespans, not absolute deadlines**, so a restore can re-base
   time; and keep snapshots in a sorted map so "latest at or before T" is `O(log B)`.

The interview signal that scores this problem: state the forward-only-timestamps
assumption, choose the cheaper structure *because of it*, and name the tradeoffs
(sorted map's `O(log F)` lookups; backups as full copies) before you're asked.
