// An enum is a value that is exactly ONE of several variants.
// Define Direction with four variants, then match one to its name.

enum Direction {
    North,
    East,
    South,
    West,
}

fn heading(d: Direction) -> &'static str {
    // Match `d` and return the matching name as text.
    // One arm per variant:  Direction::North => "North",  ...
    // your code here
}

fn main() {
    let going = Direction::West;
    println!("heading {}", heading(going));   // should print: heading West
}
