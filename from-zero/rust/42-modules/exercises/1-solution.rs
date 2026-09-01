// A module is a named box for items, with a wall around it that is CLOSED by
// default. The wall is one-way glass: a child can see everything in its
// parents, including their private items; a parent can only see what the child
// marked `pub`.
//
// Run with:  rustc --edition 2024 1-solution.rs && ./1-solution

// Private to the crate root — and yet every module below can read it, because
// they are all descendants of the root.
const SITE_NAME: &str = "north ridge";

mod weather {
    pub mod sensors {
        // Public: the wall has a window here.
        pub fn read_celsius() -> f64 {
            21.5
        }

        // Private: nobody outside `sensors` can call this, not even `weather`.
        fn calibration_offset() -> f64 {
            0.4
        }

        pub fn read_calibrated() -> f64 {
            // Inside the module, privacy is irrelevant — this is home.
            read_celsius() + calibration_offset()
        }

        pub fn describe() -> String {
            // `super::` steps up one module: sensors -> weather. `label` is
            // PRIVATE in `weather`, and we can still call it. Looking UP is
            // always allowed.
            //
            // Note you still have to spell the path. Being allowed to see a
            // name is not the same as having it in scope — `SITE_NAME` alone
            // is E0425, `crate::SITE_NAME` is fine.
            format!("{}, {}", super::label(), crate::SITE_NAME)
        }
    }

    // Private to `weather`. `main` cannot call this; `sensors` can.
    fn label() -> &'static str {
        "hourly reading"
    }

    pub fn report() -> String {
        // `crate::` starts from the crate root, wherever we happen to be.
        format!(
            "{}: {:.1}C at {}",
            label(),
            sensors::read_calibrated(),
            crate::SITE_NAME
        )
    }
}

// `use` does not open anything and does not move anything. It only shortens a
// path for the rest of this file — and the item must already be `pub`.
use weather::sensors::read_calibrated;
use weather::sensors as probe;

fn main() {
    // The long way: spell the whole path from the root.
    println!("{}", weather::report());
    println!("raw:        {:.1}C", weather::sensors::read_celsius());

    // The short ways, thanks to the two `use` lines above.
    println!("calibrated: {:.1}C", read_calibrated());
    println!("renamed:    {:.1}C", probe::read_calibrated());

    println!("{}", probe::describe());

    // Each of these is a compile error, not a runtime one. Uncomment to see:
    //
    // weather::label();
    //   error[E0603]: function `label` is private
    //   -> a parent cannot reach into a child's private items.
    //
    // weather::sensors::calibration_offset();
    //   error[E0603]: function `calibration_offset` is private
    //   -> and neither can a grandparent.
    println!();
    println!("privacy is checked at compile time — the errors above never run");
}

// hourly reading: 21.9C at north ridge
// raw:        21.5C
// calibrated: 21.9C
// renamed:    21.9C
// hourly reading, north ridge
//
// privacy is checked at compile time — the errors above never run
