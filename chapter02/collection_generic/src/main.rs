use std::ops::Add;

trait Summable<T> {
    fn sum(&self) -> T;
}

impl<T: Add<Output = T> + Default + Copy> Summable<T> for Vec<T> {
    fn sum(&self) -> T {
        let mut total = T::default();
        for &item in self {
            total = total + item;
        }
        total
    }
}

fn main() {
    let numbers = vec![11, 22, 3, 90];
    println!("i32 sum: {}", numbers.sum());  // Prints: i32 sum: 126

    let floats = vec![111.1, 12.12, 311.3];
    println!("f64 sum: {}", floats.sum());  // Prints: f64 sum: 434.52
}
