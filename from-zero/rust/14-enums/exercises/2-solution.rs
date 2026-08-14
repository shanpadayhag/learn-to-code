enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}

fn area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => 3.14159 * radius * radius,
        Shape::Rectangle(width, height) => width * height,
    }
}

fn main() {
    let round = Shape::Circle(2.0);
    let boxy = Shape::Rectangle(3.0, 4.0);
    println!("{}", area(round));
    println!("{}", area(boxy));
}
