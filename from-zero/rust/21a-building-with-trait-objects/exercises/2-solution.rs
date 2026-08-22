// Interlude 21a · Exercise 2 — the shape factory

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
    match name {
        "rectangle" => Some(Box::new(Rectangle { width: 5.0, height: 3.0 })),
        "circle" => Some(Box::new(Circle { radius: 2.0 })),
        _ => None,
    }
}

fn main() {
    for name in ["rectangle", "circle", "hexagon"] {
        match make_shape(name) {
            Some(shape) => println!("{}: {:.2}", name, shape.area()),
            None => println!("{}: unknown shape", name),
        }
    }
}
