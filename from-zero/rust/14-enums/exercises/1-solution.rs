enum Direction {
    North,
    East,
    South,
    West,
}

fn heading(d: Direction) -> &'static str {
    match d {
        Direction::North => "North",
        Direction::East => "East",
        Direction::South => "South",
        Direction::West => "West",
    }
}

fn main() {
    let going = Direction::West;
    println!("heading {}", heading(going));
}
