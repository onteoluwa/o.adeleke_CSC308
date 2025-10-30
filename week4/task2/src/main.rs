//Create a Circle struct with a radius field and implement methods to calculate area and circumference.
fn main() {
   let circle = Circle { radius: 5.0 };
    println!("Area: {}", Circle.area());
    println!("Circumference: {}", Circle.circumference());
}

struct Circle {
    radius: f64,
}
