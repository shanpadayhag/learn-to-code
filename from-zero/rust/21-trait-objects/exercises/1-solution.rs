// Concept 21 · Exercise 1 — one loop over a MIXED pile

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
    let animals: Vec<Box<dyn Greet>> = vec![Box::new(Dog), Box::new(Cat), Box::new(Dog)];
    for animal in &animals {
        println!("{}", animal.hello());
    }
}
