// A TRAIT is a named set of abilities. You write it once, then each type promises
// to provide those abilities with `impl Trait for Type`. Any type that implements
// the trait can be used wherever that trait is asked for.
//
// Define a trait `Greet` with one method `hello(&self) -> String`, then implement
// it for BOTH `Dog` and `Cat` so each returns its own greeting.

trait Greet {
    // A method signature with NO body — every implementer must fill it in.
    fn hello(&self) -> String;
}

struct Dog;
struct Cat;

// impl Greet for Dog { ... }   — make hello() return "Woof!"
// impl Greet for Cat { ... }   — make hello() return "Meow!"
// your code here

fn main() {
    let d = Dog;
    let c = Cat;
    println!("{}", d.hello());   // Woof!
    println!("{}", c.hello());   // Meow!
}
