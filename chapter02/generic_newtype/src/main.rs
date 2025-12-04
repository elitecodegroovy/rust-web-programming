use std::fmt;

struct MyString(String);

impl fmt::Display for MyString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[MyString]: {}", self.0)
    }
}

fn do_scope() {
    let x = 10;

    {
        let y = 20;
        println!("x = {}", x); // OK: x is in outer scope
        println!("y = {}", y); // OK: y is in inner scope
    }

    // println!("y = {}", y); // ERROR: y does not live here
    do_scope2();
}

fn do_scope2() {
    let x = 5;
    println!("x = {}", x); // 5

    {
        let x = x + 1; // shadows outer x
        println!("x = {}", x); // 6
    }

    println!("x = {}", x); // 5 (outer x is unchanged)
    do_scope3();
}

fn do_scope3() {
    let mut s = String::from("Rust");

    {
        let r = &mut s;
        r.push_str(" Web Programming");
        println!("r = {}", r); // valid here
    } // `r` goes out of scope here, so `s` can be used again

    println!("s = {}", s); // now valid to use s again
    do_scope4();
}

fn do_scope4() {
    let s1 = String::from("Rust");
    let s2 = s1; // s1 is moved to s2

    // println!("{}", s1); // ❌ Error: s1 is no longer valid
    println!("{}", s2); //
    do_scope5();
}

fn do_scope5() {
    let mut s = String::from("Rust");

    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s;
    // ❌ Error: can't borrow mutably when immutable borrows exist

    println!("{}, {}", r1, r2);

    struct Book<'a> {
        title: &'a str,
    }

    let title = String::from("Rust Book");
    let book = Book { title: &title };

    println!("Book title: {}", book.title);
    // book and title go out of scope here safely
}

fn main() {
    let s = MyString("Rust!".to_string());
    println!("{}", s); // [MyString]: Hello!

    do_scope();

}
