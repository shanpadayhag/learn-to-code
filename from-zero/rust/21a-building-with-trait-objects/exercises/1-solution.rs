// Interlude 21a · Exercise 1 — add a shape to the pile

trait Shape {
    fn area(&self) -> f64;
}

struct Rectangle { width: f64, height: f64 }
struct Circle { radius: f64 }
struct Triangle { base: f64, height: f64 }

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

impl Shape for Triangle {
    fn area(&self) -> f64 {
        self.base * self.height / 2.0
    }
}

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Rectangle { width: 5.0, height: 3.0 }),
        Box::new(Circle { radius: 2.0 }),
        Box::new(Triangle { base: 4.0, height: 3.0 }),
    ];

    for shape in &shapes {
        println!("Area: {:.2}", shape.area());
    }
}
