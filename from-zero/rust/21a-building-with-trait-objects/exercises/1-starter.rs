// Interlude 21a · Exercise 1 — add a shape to the pile
//
// Below is a working trait + two shapes, and a Vec that mixes them and
// prints each area (this is the Concept 21 pattern). Your job: add a
// THIRD shape, a Triangle, and drop one into the pile.
//
//   Triangle area = base * height / 2
//
// With Triangle { base: 4.0, height: 3.0 } added, the program should print:
//   Area: 15.00
//   Area: 12.57
//   Area: 6.00

trait Shape {
    fn area(&self) -> f64;
}

struct Rectangle { width: f64, height: f64 }
struct Circle { radius: f64 }
// your code here (1 of 2): add a `struct Triangle { base, height }`

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
// your code here (2 of 2): add `impl Shape for Triangle` with its area formula

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Rectangle { width: 5.0, height: 3.0 }),
        Box::new(Circle { radius: 2.0 }),
        // ...and add your Triangle here
    ];

    for shape in &shapes {
        println!("Area: {:.2}", shape.area());
    }
}
