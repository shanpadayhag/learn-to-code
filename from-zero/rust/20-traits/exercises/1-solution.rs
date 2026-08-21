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
    let d = Dog;
    let c = Cat;
    println!("{}", d.hello());
    println!("{}", c.hello());
}
