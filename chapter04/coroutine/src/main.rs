


#[derive(Debug)]
enum ListEnum {
    Cons(i32, Box<ListEnum>),
    Nil,
}

use crate::ListEnum::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list enum : {:?}", list);
}
