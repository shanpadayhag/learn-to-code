// Variants can carry data. A Shape is EITHER a circle with a radius
// OR a rectangle with a width and height.

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}

fn area(shape: Shape) -> f64 {
    // Match `shape`. Pull the data out of each variant into names,
    // then compute the area:
    //   Circle(radius)          => 3.14159 * radius * radius
    //   Rectangle(width, height) => width * height
    // your code here
}

fn main() {
    let round = Shape::Circle(2.0);
    let boxy = Shape::Rectangle(3.0, 4.0);
    println!("{}", area(round));   // should print: 12.56636
    println!("{}", area(boxy));    // should print: 12
}
