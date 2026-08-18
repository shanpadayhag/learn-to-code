struct Wrapper<T> {
    value: T,
}

fn main() {
    let number = Wrapper { value: 42 };
    let text = Wrapper { value: "hi" };

    println!("{}", number.value);
    println!("{}", text.value);
}
