// Build a tiny iterator BY HAND so you can feel the read-then-advance step.
//
// `CountUp` should hand out 0, 1, 2, ... up to (but not including) `limit`.
// It stores only ONE number: where it currently is. Each `.next()` reads that
// number, moves the position forward by one, and hands the read value back.
// When the position reaches `limit`, there is nothing left, so it hands back
// `None`.

struct CountUp {
    current: u32,
    limit: u32,
}

impl Iterator for CountUp {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.current == self.limit {
            return None;
        }

        let value_to_hand_back = self.current;
        self.current += 1;

        Some(value_to_hand_back)
    }
}

fn main() {
    let counter = CountUp { current: 0, limit: 3 };

    for number in counter {
        println!("{number}");
    }
    // Expected output (one per line): 0  1  2
}
