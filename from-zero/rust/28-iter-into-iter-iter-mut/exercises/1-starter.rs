// `.iter_mut()` yields `&mut T` — a mutable reference to each element — so you
// can edit the collection in place. Write through the reference with `*n`.

fn main() {
    let mut prices = vec![100, 200, 300];

    // Use .iter_mut() to add 5 to every price IN PLACE:
    //   for p in prices.iter_mut() { *p += 5; }
    // your code here

    // println!("{prices:?}");   // should print: [105, 205, 305]
}
