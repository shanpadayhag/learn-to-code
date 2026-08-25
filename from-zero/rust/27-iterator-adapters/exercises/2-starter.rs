// Chain adapters, then finish with a consumer. `.filter` keeps items where the
// closure is true; `.map` transforms each; `.sum()` adds them all into one value.

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Keep the even numbers, triple each, then sum them into one i32:
    //   .into_iter().filter(...).map(...).sum()
    // let total: i32 = ... ;
    // your code here

    // println!("{total}");   // should print: 90
}
