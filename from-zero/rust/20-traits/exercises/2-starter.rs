// Back in Concept 19, this function REFUSED to compile:
//
//     fn larger<T>(a: T, b: T) -> T {
//         if a > b { a } else { b }   // error: can't compare two T with `>`
//     }
//
// Rust knew nothing about T, so it wouldn't let you use `>`. A TRAIT BOUND fixes
// it: `T: PartialOrd` promises "T is a type that can be compared with < and >".
// With that promise in place, `>` is allowed.
//
// Finish `larger` by giving T the PartialOrd bound, then return the bigger value.

fn larger<T>(a: T, b: T) -> T {
    // add the bound `T: PartialOrd` above, then:
    // return a if it is greater than b, otherwise b
    // your code here
}

fn main() {
    println!("{}", larger(3, 9));          // 9
    println!("{}", larger(2.5, 1.5));      // 2.5
    println!("{}", larger("apple", "pear")); // pear   (strings compare alphabetically)
}
