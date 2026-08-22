// Interlude 21a · Exercise 2 — the shape factory
//
// This is the exact piece that froze you: a function that takes a NAME
// and hands back "some Shape, if the name is valid." That return type is
// `Option<Box<dyn Shape>>` — a boxed shape (Concept 21) that might be
// missing (Concept 15).
//
// Finish `make_shape` so it returns:
//   "rectangle" -> a Rectangle 5.0 x 3.0
//   "circle"    -> a Circle radius 2.0
//   anything else -> None
//
// Expected output:
//   rectangle: 15.00
//   circle: 12.57
//   hexagon: unknown shape

trait Shape {
    fn area(&self) -> f64;
}

struct Rectangle { width: f64, height: f64 }
struct Circle { radius: f64 }

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

fn make_shape(name: &str) -> Option<Box<dyn Shape>> {
    // your code here: match on `name`, returning Some(Box::new(...)) or None
    None
}

fn main() {
    for name in ["rectangle", "circle", "hexagon"] {
        match make_shape(name) {
            Some(shape) => println!("{}: {:.2}", name, shape.area()),
            None => println!("{}: unknown shape", name),
        }
    }
}
