fn describe(a: Option<i32>, b: Option<i32>) {
    if let Some((x, y)) = a.zip(b) {
        println!("sum: {}", x + y);
    } else {
        println!("missing");
    }
}

fn main() {
    describe(Some(2), Some(3));
    describe(Some(2), None);
}
