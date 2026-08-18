// Structs can be generic too. `struct Wrapper<T> { value: T }` holds a value of
// ANY type T. When you build one, Rust reads the value you give it to decide what
// T is: a Wrapper holding 42 is a Wrapper<i32>, one holding "hi" is a Wrapper<&str>
// — the SAME definition, stamped out for each type by the compiler.

// Define the generic struct here:
// your code here

fn main() {
    let number = Wrapper { value: 42 };   // Wrapper<i32>
    let text = Wrapper { value: "hi" };   // Wrapper<&str>

    println!("{}", number.value);
    println!("{}", text.value);
    // Expected:
    //   42
    //   hi
}
