fn main() {
    let a = 5;
    let b = 8;
    let ra = &a;
    let rb = &b;

    let pair: Vec<i32> = vec![*ra, *rb];
    println!("{:?}", pair);
}
