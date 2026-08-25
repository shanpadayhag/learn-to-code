// Watch the reference count rise and fall.
//
// Make an Rc, add owners with Rc::clone, and print Rc::strong_count at each
// step. Notice you never copy the string itself — only the count changes.

use std::rc::Rc;

fn main() {
    let first = Rc::new(String::from("shared text"));
    println!("after new:          {}", /* your code here: strong_count of first */); // 1

    let second = /* your code here: clone another owner from first */;
    println!("after 2nd owner:    {}", Rc::strong_count(&first)); // 2

    {
        let third = /* your code here: clone a third owner */;
        println!("inside inner block: {}", Rc::strong_count(&first)); // 3
        println!("third reads: {third}");
    } // third goes out of scope here

    println!("after block ended:  {}", Rc::strong_count(&first)); // 2
    println!("first and second still read: {first} / {second}");
}
