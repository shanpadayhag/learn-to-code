// The payoff of the wall: a PRIVATE field is a promise nobody outside can
// break. Build a `Gauge` that keeps a running total next to its readings, so
// the two can never disagree.
//
// `pub struct` does NOT make the fields public. Each field needs its own `pub`
// — which means leaving `pub` off is how you protect an invariant.
//
// Run with:  rustc --edition 2024 2-starter.rs && ./2-starter

// 1. Write a module `gauge` holding a PUBLIC struct with PRIVATE fields:
//
//    pub struct Gauge {
//        readings: Vec<f64>,
//        total: f64,      // invariant: always equals readings.iter().sum()
//    }

// 2. In an `impl Gauge` block, give it four public methods:
//      new()      -> Self          empty readings, total 0.0
//      record(&mut self, f64)      push the reading AND add it to total
//      count(&self) -> usize
//      average(&self) -> Option<f64>    None when empty, else total / count
//
//    `average` is O(1) — no summing — because `record` did the work on the
//    way in. That shortcut is only sound because nothing outside the module
//    can touch `total`.

// 3. Add `hottest(&self) -> Option<f64>` using the iterator tools from
//    Phase 7: .iter().copied().fold(None, ...) or .max_by(...).

// 4. Add a free function INSIDE the module:
//
//    pub fn invariant_holds(gauge: &Gauge) -> bool
//
//    summing `gauge.readings` and comparing to `gauge.total`. It can reach the
//    private fields because it lives in the same module — privacy is per
//    MODULE, not per type.

// 5. Below the module, re-export the type:  pub use gauge::Gauge;
//    Now the root has a second name for it. Nothing moved.

// 6. In `main`: build a gauge, print the average while it is still empty,
//    record 19.5, 22.0, 20.5 and 24.0, then print count, average, hottest,
//    and gauge::invariant_holds(&gauge).

// 7. Now try to break it, one line at a time, and read each error:
//      gauge.total = 999.0;
//      gauge.readings.push(50.0);
//      let broken = Gauge { readings: vec![1.0], total: 99.0 };
//    The second is the interesting one: it would add a reading without
//    updating the total, and `average` would quietly start lying. The wall is
//    what makes that unwritable rather than merely unwise.

fn main() {
    // your code here
}
