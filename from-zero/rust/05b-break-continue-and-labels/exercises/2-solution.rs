fn main() {
    'outer: for a in 1..=3 {
        for b in 1..=3 {
            if a == b {
                continue 'outer;
            }
            println!("{a},{b}");
        }
    }
}
