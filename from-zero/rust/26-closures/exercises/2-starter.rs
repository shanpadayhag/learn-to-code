// Iterator adapters take a closure describing what to do with each item —
// and the closure can capture a local to decide. `.filter` keeps items for
// which the closure returns true; `.count()` counts what's left.

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let min = 3;

    // Count how many numbers are >= min, using .into_iter().filter(...).count()
    // with a closure that captures `min`:
    //   numbers.into_iter().filter(|&n| n >= min).count()
    // your code here

    // println!("{count}");   // should print: 3
}
