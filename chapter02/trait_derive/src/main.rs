#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1.clone(); // 自动实现了 Clone
    println!("{:?}", p2); // 自动实现了 Debug
    println!("{}", p1 == p2); // 自动实现了 PartialEq
}
