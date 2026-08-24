// Two Options, and you only care when BOTH are Some. Instead of nesting one
// `if let` inside another, put both in a tuple and match the pair at once:
//   if let (Some(x), Some(y)) = (a, b) { ... }

fn describe(a: Option<i32>, b: Option<i32>) {
    // Tuple-match (a, b). If both are Some, print "sum: {x+y}".
    // Otherwise print "missing".
    // your code here
}

fn main() {
    describe(Some(2), Some(3));   // should print: sum: 5
    describe(Some(2), None);      // should print: missing
}
