use std::rc::Rc;

fn show(name: Rc<String>) {
    println!("running: {name}");
}

fn main() {
    let app_name = Rc::new(String::from("Folio"));

    show(Rc::clone(&app_name));
    show(Rc::clone(&app_name));

    println!("still have it: {app_name}");
    println!("final count: {}", Rc::strong_count(&app_name)); // 1
}
