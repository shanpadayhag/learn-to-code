# Lazy Expiration

**In one line:** instead of actively deleting things the moment they expire, you
just **write down when each thing dies** and check that deadline *when someone
looks* — so expiry costs nothing until it's actually relevant.

## Plain explanation
Imagine a fridge full of food, each item with a "use by" date on the label. There
are two ways to keep expired food out of your meals:

- **Eager:** set an alarm for every single item and, the instant one expires, get up
  and throw it out. Now you're running to the fridge all day, mostly to toss things
  nobody was going to eat anyway.
- **Lazy:** do nothing on a timer. When you actually reach for something, *then*
  glance at its date — if it's past, treat it as not there. You only ever pay
  attention to an item at the moment you care about it.

Lazy expiration is the second way. A stored value carries a **death time**. Nothing
watches the clock; the clock never has to *do* anything. Every read simply asks "is
this still alive right now?" — present, and either it has no deadline or the current
time is still before it.

## Why you care
The eager approach spends effort proportional to *everything that could expire*, on
a schedule, whether or not anyone would have noticed. The lazy approach spends effort
only proportional to *what you actually read*. When expirations vastly outnumber
reads-of-expired-things (the common case), lazy is dramatically cheaper — and far
simpler: no background timer, no cleanup thread, no sweep.

The catch is **memory**: a value that expired but is never read again just sits
there, dead weight, until something happens to overwrite or touch it. If that matters,
you bolt on *occasional* reclamation (drop it when a read notices it's dead, or sweep
in the background now and then) — but the correctness of every *read* already comes
from the check-on-access, not from any cleanup running.

This pairs naturally with a [sorted map](sorted-map.md) or
[hash map](hash-map.md): the map stores the value plus its deadline, and the liveness
check is a single comparison layered on top of a normal lookup — so it adds no change
to the lookup's Big-O.

## Quick examples
A field written at time 5 with a 10-tick TTL stores `expires_at = 15`. No alarm is
set. Later reads just compare:

| read at time | `t < 15`? | result |
|---|---|---|
| 12 | yes | value is returned |
| 15 | no | treated as absent (expired) |
| 99 | no | treated as absent |

Nobody ever "expired" the field — the read at 15 simply *computed* that it was past
its deadline. Session tokens, cache entries, and rate-limit windows are all real-world
things usually expired this way.

## Related
- [Sorted Map](sorted-map.md)
- [Hash Map](hash-map.md)

## Shows up in
- [In-Memory Database](../patterns/in-memory-database/README.md)
