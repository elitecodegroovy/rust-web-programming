fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn call_max() {
    let max_int = max(15, 10);                 // Works with integers: returns 15
    let max_float = max(3.14, 3.71);           // Works with floats: returns 3.71
    let max_str = max("enables", "function"); // Works with strings: returns "function"

    println!("max int  {}", max_int);
    println!("max float {}", max_float);
    println!("max string {}", max_str);
}

use std::fmt::Display;

struct Wrapper<T> {
    value: T,
}

trait Printable {
    fn print(&self);
}

impl<T: Display> Printable for Wrapper<T> {
    fn print(&self) {
        println!("{}", self.value);
    }
}

fn call_generic_impl() {
    let wrapped_int = Wrapper { value: 59 };
    wrapped_int.print();  // Prints: 59

    let wrapped_str = Wrapper { value: "Rust" };
    wrapped_str.print(); // Prints: Rust
}

fn main() {
    call_max();

    call_generic_impl();
}
