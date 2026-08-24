// Same task, the other clean way: `a.zip(b)` welds two Options into one
// `Option<(i32, i32)>` — Some only when BOTH were Some. Then open that single
// Option with one `if let Some((x, y))`.

fn describe(a: Option<i32>, b: Option<i32>) {
    // Use a.zip(b), then `if let Some((x, y)) = ...` to print "sum: {x+y}",
    // else "missing".
    // your code here
}

fn main() {
    describe(Some(2), Some(3));   // should print: sum: 5
    describe(Some(2), None);      // should print: missing
}
