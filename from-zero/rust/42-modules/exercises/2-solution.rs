// The payoff of the wall: a PRIVATE field is a promise nobody outside can
// break. `Gauge` keeps a running total alongside its readings, and the two are
// guaranteed to agree — not because the code is careful, but because the only
// way to add a reading is `record`, and `record` updates both.
//
// `pub struct` does NOT make the fields public. Each field needs its own `pub`.
//
// Run with:  rustc --edition 2024 2-solution.rs && ./2-solution

mod gauge {
    // Public type, private insides.
    pub struct Gauge {
        readings: Vec<f64>,
        total: f64, // invariant: always equal to readings.iter().sum()
    }

    impl Gauge {
        pub fn new() -> Self {
            Gauge {
                readings: Vec::new(),
                total: 0.0,
            }
        }

        // The only door in. Both fields move together, so the invariant holds.
        pub fn record(&mut self, celsius: f64) {
            self.readings.push(celsius);
            self.total += celsius;
        }

        pub fn count(&self) -> usize {
            self.readings.len()
        }

        // O(1), because the total was maintained on the way in. That shortcut
        // is only safe because nothing outside can touch `total`.
        pub fn average(&self) -> Option<f64> {
            if self.readings.is_empty() {
                None
            } else {
                Some(self.total / self.readings.len() as f64)
            }
        }

        pub fn hottest(&self) -> Option<f64> {
            self.readings.iter().copied().fold(None, |best, reading| match best {
                Some(current) if current >= reading => Some(current),
                _ => Some(reading),
            })
        }
    }

    // Only inside this module can the invariant be inspected directly.
    pub fn invariant_holds(gauge: &Gauge) -> bool {
        let recomputed: f64 = gauge.readings.iter().sum();
        (recomputed - gauge.total).abs() < 1e-9
    }
}

// A re-export: `Gauge` now also lives at the crate root, so callers write
// `Gauge` instead of `gauge::Gauge`. The item did not move — this is a second
// name for the same thing.
pub use gauge::Gauge;

fn main() {
    let mut gauge = Gauge::new();
    println!("empty average: {:?}", gauge.average());

    for reading in [19.5, 22.0, 20.5, 24.0] {
        gauge.record(reading);
    }

    println!("count:   {}", gauge.count());
    println!("average: {:.2}", gauge.average().unwrap());
    println!("hottest: {:.1}", gauge.hottest().unwrap());
    println!("invariant holds: {}", gauge::invariant_holds(&gauge));

    // Every one of these is a compile error. Uncomment to meet them:
    //
    // gauge.total = 999.0;
    //   error[E0616]: field `total` of struct `Gauge` is private
    //
    // gauge.readings.push(50.0);
    //   error[E0616]: field `readings` of struct `Gauge` is private
    //   -> and this is the one that matters: it would push a reading WITHOUT
    //      updating `total`, and `average` would quietly start lying.
    //
    // let broken = Gauge { readings: vec![1.0], total: 99.0 };
    //   error[E0451]: fields `readings` and `total` of struct `Gauge` are private
    //   -> you cannot even build one by hand. `new` is the only way in.
    println!();
    println!("nothing outside the module can desynchronise the two fields");
}

// empty average: None
// count:   4
// average: 21.50
// hottest: 24.0
// invariant holds: true
//
// nothing outside the module can desynchronise the two fields
