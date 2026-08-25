// Feel the difference between `.take_while` and `.filter`.
//
// Both take a test. But:
//   .filter(test)     -> SKIPS the items that fail, keeps walking to the end
//   .take_while(test) -> STOPS for good at the first item that fails
//
// The list is [2, 4, 5, 6, 8]. Reading left to right, 5 is the first odd one.
//   .filter(even)     keeps 2, 4, 6, 8   (it skips the 5 and keeps going)
//   .take_while(even) keeps 2, 4         (it hits the 5 and stops — 6 and 8 never get a turn)
//
// `.iter()` walks the Vec by borrowing, so it hands out `&i32` (references).
// `.copied()` turns each borrowed number into a plain owned `i32`, so the rest
// of the chain — and the final `Vec<i32>` — deals in real numbers, not borrows.

fn main() {
    let numbers = vec![2, 4, 5, 6, 8];

    let filtered: Vec<i32> = numbers
        .iter()
        .copied()
        .filter(|n| n % 2 == 0)
        .collect();

    let taken: Vec<i32> = numbers
        .iter()
        .copied()
        .take_while(|n| n % 2 == 0)
        .collect();

    println!("filter:     {filtered:?}");
    println!("take_while: {taken:?}");
    // Expected output:
    // filter:     [2, 4, 6, 8]
    // take_while: [2, 4]
}
