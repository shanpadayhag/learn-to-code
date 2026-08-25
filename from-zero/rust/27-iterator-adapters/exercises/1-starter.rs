// `.map(closure)` transforms every item; `.collect()` gathers the results into
// a collection. The type annotation on the left tells `.collect` what to build.

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Build a Vec<i32> of each number squared (n * n) using
    //   .iter().map(...).collect()
    // let squares: Vec<i32> = ... ;
    // your code here

    // println!("{squares:?}");   // should print: [1, 4, 9, 16, 25]
}
