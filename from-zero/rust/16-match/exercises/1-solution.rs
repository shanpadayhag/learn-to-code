fn grade(score: u32) -> char {
    match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F',
    }
}

fn main() {
    println!("{}", grade(95));
    println!("{}", grade(83));
    println!("{}", grade(42));
}
