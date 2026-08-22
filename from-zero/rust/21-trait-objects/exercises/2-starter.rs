// Concept 21 · Exercise 2 — a function over any mix of shapes
//
// A trait object isn't only for printing — the method can COMPUTE and
// return a value too. Write `total_area`, which takes a slice of mixed
// shapes behind `dyn Shape` and adds up their areas with one loop.
//
// Circle area  = 3.14159 * r * r
// Rectangle    = width * height
//
// With a Circle(radius 2.0) and a Rectangle(3.0 x 4.0):
//   circle    = 3.14159 * 2 * 2 = 12.56636
//   rectangle = 3 * 4           = 12
//   total     = 24.56636  ->  printed as "24.57"

trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14159 * self.radius * self.radius
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// your code here: complete `total_area` so it sums the area of every shape.
fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    0.0
}

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 2.0 }),
        Box::new(Rectangle { width: 3.0, height: 4.0 }),
    ];
    println!("{:.2}", total_area(&shapes));
}
