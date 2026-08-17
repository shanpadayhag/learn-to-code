fn squares(n: u32) -> Vec<u32> {
    let mut result = Vec::new();
    let mut i = 1;
    while i <= n {
        result.push(i * i);
        i += 1;
    }
    result
}

fn main() {
    let s = squares(5);
    println!("{:?}", s);

    let mut total = 0;
    for &value in &s {
        total += value;
    }
    println!("{total}");
}
