// A module is a named box for items, with a wall around it that is CLOSED by
// default. The wall is one-way glass: a child can see everything in its
// parents, including their private items; a parent can only see what the child
// marked `pub`.
//
// Run with:  rustc --edition 2024 1-starter.rs && ./1-starter

// 1. At the top of the file (the crate root), declare a private constant:
//
//    const SITE_NAME: &str = "north ridge";
//
//    Private to the root — and yet every module you write below will be able
//    to read it. Predict why before you get to step 4.

// 2. Write a module `weather` containing a nested module `sensors`:
//
//    mod weather {
//        pub mod sensors {
//            pub fn read_celsius() -> f64 { 21.5 }
//            fn calibration_offset() -> f64 { 0.4 }        // note: no pub
//            pub fn read_calibrated() -> f64 { ... }       // celsius + offset
//        }
//    }
//
//    `read_calibrated` calls `calibration_offset` with no path and no
//    permission problem, because inside the module privacy does not exist.

// 3. Give `weather` a PRIVATE fn `label() -> &'static str` returning
//    "hourly reading", and a `pub fn report() -> String` that formats
//    label(), sensors::read_calibrated() and `crate::SITE_NAME` into one line.
//    `crate::` always starts from the crate root, wherever you are standing.

// 4. Add `pub fn describe() -> String` inside `sensors` that calls
//    `super::label()` — one step UP the tree, into a private item of its
//    parent. It compiles. Looking up is always allowed.
//
//    Then try writing just `SITE_NAME` instead of `crate::SITE_NAME` in it.
//    You get E0425 "cannot find value". Work out why that is NOT a privacy
//    error — being allowed to see a name is a different thing from having it
//    in scope.

// 5. Above `main`, shorten two paths:
//
//    use weather::sensors::read_calibrated;
//    use weather::sensors as probe;
//
//    `use` opens nothing and moves nothing — the item must already be `pub`.
//    It only makes a shorter name for the rest of this file.

// 6. In `main`, print: weather::report(), the raw reading by its full path,
//    the calibrated reading through the short name, the same through `probe`,
//    and probe::describe().

// 7. Finally, add `weather::label();` to `main` and read the error. Then try
//    `weather::sensors::calibration_offset();`. Both are E0603, and both are
//    caught at COMPILE time — a wall the program never even runs into.

fn main() {
    // your code here
}
