// `match` on an enum both PICKS the variant and PULLS its data out into names.
// A `Quarter` carries which state it's from; the other coins carry nothing.

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String),
}

fn value_in_cents(coin: Coin) -> u32 {
    // Match `coin`. Return the value of each: Penny 1, Nickel 5, Dime 10,
    // Quarter 25. For a Quarter, first print "a quarter from {state}!" using
    // the state bound out of the variant, then give back 25. (An arm can be a
    // `{ ... }` block whose last line is its value.)
    // your code here
}

fn main() {
    println!("{}", value_in_cents(Coin::Penny));    // should print: 1
    println!("{}", value_in_cents(Coin::Nickel));   // should print: 5
    println!("{}", value_in_cents(Coin::Dime));     // should print: 10
    println!("{}", value_in_cents(Coin::Quarter(String::from("Texas"))));
    // should print:  a quarter from Texas!
    //                25
}
