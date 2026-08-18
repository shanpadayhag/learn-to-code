fn swap<T>(a: T, b: T) -> (T, T) {
    (b, a)
}

fn main() {
    println!("{:?}", swap(10, 20));
    println!("{:?}", swap("hello", "world"));
}
