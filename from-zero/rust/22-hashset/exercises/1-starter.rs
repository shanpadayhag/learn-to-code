// Concept 22 · Exercise 1 — count the UNIQUE values
//
// A HashSet keeps each value at most once — insert a duplicate and it's
// simply ignored. So the number of unique values is just the set's len().
//
// Insert every number below into a HashSet, then print how many DIFFERENT
// values there were.
//
// numbers  = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5]
// uniques  = {1, 2, 3, 4, 5, 6, 9}  ->  7

use std::collections::HashSet;

fn main() {
    let numbers = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];

    // your code here:
    // 1. make an empty HashSet<i32>
    // 2. insert every value in `numbers`
    // 3. println! the set's len()
}
