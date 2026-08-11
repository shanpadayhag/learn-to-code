struct Counter {
    count: i32,
}

fn main() {
    let mut c = Counter { count: 0 };

    c.count += 5;
    c.count += 1;

    println!("{}", c.count);
}
