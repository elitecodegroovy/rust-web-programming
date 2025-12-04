trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn print_area(shape: &dyn Area) {
    println!("Area: {:.2}", shape.area());
}

fn main() {
    let c = Circle { radius: 5.0 };
    let r = Rectangle { width: 8.0, height: 9.0 };

    print_area(&c);
    print_area(&r);
}
