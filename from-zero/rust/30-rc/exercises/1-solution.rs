use std::rc::Rc;

fn main() {
    let first = Rc::new(String::from("shared text"));
    println!("after new:          {}", Rc::strong_count(&first)); // 1

    let second = Rc::clone(&first);
    println!("after 2nd owner:    {}", Rc::strong_count(&first)); // 2

    {
        let third = Rc::clone(&first);
        println!("inside inner block: {}", Rc::strong_count(&first)); // 3
        println!("third reads: {third}");
    } // third goes out of scope here

    println!("after block ended:  {}", Rc::strong_count(&first)); // 2
    println!("first and second still read: {first} / {second}");
}
