struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let boxed_number = Box::new(5);
    let total = *boxed_number + 100;
    println!("total: {total}"); // 105

    let boxed_point = Box::new(Point { x: 3, y: 4 });
    println!("point is ({}, {})", boxed_point.x, boxed_point.y); // (3, 4)
}
