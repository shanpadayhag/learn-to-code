// Box a value and read it back — two ways to reach inside a box.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // 1) Put an i32 on the heap, then dereference with * to add 100 to it.
    let boxed_number = Box::new(5);
    let total = /* your code here: follow the box, then + 100 */;
    println!("total: {total}"); // expect 105

    // 2) Box a Point, then read a field WITHOUT writing * — field access
    //    auto-dereferences through the box for you.
    let boxed_point = Box::new(Point { x: 3, y: 4 });
    println!("point is ({}, {})", /* your code here: x */, /* and y */);  // expect (3, 4)
}
