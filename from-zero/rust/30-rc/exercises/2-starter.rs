// One config, many readers.
//
// A single app-name lives in an Rc. `show` takes ownership of an Rc and prints
// it. Because you hand it a fresh Rc::clone each time (a cheap count bump, not a
// copy), the original stays usable afterwards.

use std::rc::Rc;

fn show(name: Rc<String>) {
    println!("running: {name}");
}

fn main() {
    let app_name = Rc::new(String::from("Folio"));

    show(/* your code here: a fresh clone owner */);
    show(/* your code here: another fresh clone owner */);

    // The original is still ours — clone never took it away.
    println!("still have it: {app_name}");
    println!("final count: {}", Rc::strong_count(&app_name)); // 1
}
