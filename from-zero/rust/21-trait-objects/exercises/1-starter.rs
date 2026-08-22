// Concept 21 · Exercise 1 — one loop over a MIXED pile
//
// Concept 20 could greet a Dog OR a Cat, but never both in one list.
// Fix that: build a single Vec that holds a Dog AND a Cat AND a Dog,
// then greet every one of them with ONE loop.
//
// Expected output:
//   Woof!
//   Meow!
//   Woof!

trait Greet {
    fn hello(&self) -> String;
}

struct Dog;
struct Cat;

impl Greet for Dog {
    fn hello(&self) -> String {
        String::from("Woof!")
    }
}

impl Greet for Cat {
    fn hello(&self) -> String {
        String::from("Meow!")
    }
}

fn main() {
    // your code here:
    // 1. make `animals` a Vec<Box<dyn Greet>> holding a Dog, a Cat, and a Dog
    // 2. loop over &animals and println! each one's hello()
}
