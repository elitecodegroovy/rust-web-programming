mod schema;
mod models;
mod common;

use actix_web::{get, post, error, web, App, http::{self, header::ContentEncoding, StatusCode}, middleware::Compress, middleware::Logger, HttpResponse, HttpServer, Responder, Result, HttpRequest};
use actix_web::{ rt, body::{MessageBody, BoxBody}, http::header::ContentType, http::{header}, middleware::{from_fn, Next}, dev::{ServiceRequest, ServiceResponse}, middleware::{ErrorHandlerResponse, ErrorHandlers}};
use serde::Serialize;
use serde::Deserialize;
use std::task::Poll;
use futures::{future::ok, stream::once};

use derive_more::derive::{Display, Error};
use log::info;
use log::error as error_log;
use std::time::Duration;

use std::convert::Into;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicUsize;
use actix_ws::AggregatedMessage;
use redis::{Client, Commands, Connection};

#[allow(dead_code)]
#[derive(Debug)]
struct Number {
    value: i32,
}

impl Into<Number> for i32 {
    fn into(self) -> Number {
        Number { value: self }
    }
}

use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

use std::fmt;

#[derive(Debug)]
struct Circle {
    radius: i32
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "圆的半径 {}", self.radius)
    }
}

use std::num::ParseIntError;
use std::str::FromStr;
use futures_util::StreamExt as _;

impl FromStr for Circle {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().parse() {
            Ok(num) => Ok(Circle{ radius: num }),
            Err(e) => Err(e),
        }
    }
}

#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    // These likewise tie `u32` tuples to different names: color models.
    RGB(u32, u32, u32),
}

fn capture() {
    use std::mem;
    let color = String::from("green");
    // A closure to print `color` which immediately borrows (`&`) `color` and
    // stores the borrow and closure in the `print` variable. It will remain
    // borrowed until `print` is used the last time.
    // `println!` only requires arguments by immutable reference so it doesn't
    // impose anything more restrictive.
    let print = || println!("`color`: {}", color);

    // Call the closure using the borrow.
    print();

    // `color` can be borrowed immutably again, because the closure only holds
    // an immutable reference to `color`.
    let _reborrow = &color;
    print();

    // A move or reborrow is allowed after the final use of `print`
    let _color_moved = color;


    let mut count = 0;
    // A closure to increment `count` could take either `&mut count` or `count`
    // but `&mut count` is less restrictive so it takes that. Immediately
    // borrows `count`.
    //
    // A `mut` is required on `inc` because a `&mut` is stored inside. Thus,
    // calling the closure mutates `count` which requires a `mut`.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // Call the closure using a mutable borrow.
    inc();

    // The closure still mutably borrows `count` because it is called later.
    // An attempt to reborrow will lead to an error.
    // let _reborrow = &count;
    // ^ TODO: try uncommenting this line.
    inc();

    // The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error
    let _count_reborrowed = &mut count;


    // A non-copy type.
    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` consumes the variable so this can only be called once.
    consume();

    // A function which takes a closure as an argument and calls it.
    // <F> denotes that F is a "Generic type parameter"
    fn apply<F>(f: F) where
    // The closure takes no input and returns nothing.
        F: FnOnce() {
        // ^ TODO: Try changing this to `Fn` or `FnMut`.

        f();
    }

    // A function which takes a closure and returns an `i32`.
    fn apply_to_3<F>(f: F) -> i32 where
    // The closure takes an `i32` and returns an `i32`.
        F: Fn(i32) -> i32 {

        f(3)
    }

    let greeting = "hello";
    // A non-copy type.
    // `to_owned` creates owned data from borrowed one
    let mut farewell = "goodbye".to_owned();

    // Capture 2 variables: `greeting` by reference and
    // `farewell` by value.
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    apply(diary);

    // `double` satisfies `apply_to_3`'s trait bound
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
    // Fn: the closure uses the captured value by reference (&T)
    // FnMut: the closure uses the captured value by mutable reference (&mut T)
    // FnOnce: the closure uses the captured value by value (T)

    fn create_fn() -> impl Fn() {
        let text = "Fn".to_owned();

        move || println!("This is a: {}", text)
    }

    fn create_fnmut() -> impl FnMut() {
        let text = "FnMut".to_owned();

        move || println!("This is a: {}", text)
    }

    fn create_fnonce() -> impl FnOnce() {
        let text = "FnOnce".to_owned();

        move || println!("This is a: {}", text)
    }

    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
    call_hof();
}

fn call_hof() {
    use std::time::{Instant};
    fn is_odd(n: u32) -> bool {
        n % 2 == 1
    }

    println!("Find the sum of all the numbers with odd squares under 1000");
    let upper = 1000;

    // Imperative approach
    // Declare accumulator variable
    let start = Instant::now();
    let mut acc = 0;
    // Iterate: 0, 1, 2, ... to infinity
    for n in 0.. {
        // Square the number
        let n_squared = n * n;

        if n_squared >= upper {
            // Break loop if exceeded the upper limit
            break;
        } else if is_odd(n_squared) {
            // Accumulate value, if it's odd
            acc += n_squared;
        }
    }
    let duration = start.elapsed();
    println!("imperative style: {}, elapsed nanos: {:?}", acc, duration);

    let start2 = Instant::now();
    // Functional approach
    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|n| n * n)                             // All natural numbers squared
            .take_while(|&n_squared| n_squared < upper) // Below upper limit
            .filter(|&n_squared| is_odd(n_squared))     // That are odd
            .sum();                                     // Sum them
    println!("functional style (slower than before for loop case): {}", sum_of_squared_odd_numbers);
    let duration = start2.elapsed();
    println!("imperative style: {}, elapsed nanos: {:?}", acc, duration);

    do_rule();
}

fn do_rule() {
    // This function takes ownership of the heap allocated memory
    fn destroy_box(c: Box<i32>) {
        println!("Destroying a box that contains {}", c);

        // `c` is destroyed and the memory freed
    }

    // _Stack_ allocated integer
    let x = 5u32;

    // *Copy* `x` into `y` - no resources are moved
    let y = x;

    // Both values can be independently used
    println!("x is {}, and y is {}", x, y);

    // `a` is a pointer to a _heap_ allocated integer
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // *Move* `a` into `b`
    let b = a;
    // The pointer address of `a` is copied (not the data) into `b`.
    // Both are now pointers to the same heap allocated data, but
    // `b` now owns it.

    // Error! `a` can no longer access the data, because it no longer owns the
    // heap memory
    //println!("a contains: {}", a);
    // TODO ^ Try uncommenting this line

    // This function takes ownership of the heap allocated memory from `b`
    destroy_box(b);

    // Since the heap memory has been freed at this point, this action would
    // result in dereferencing freed memory, but it's forbidden by the compiler
    // Error! Same reason as the previous Error
    // println!("b contains: {}", b);

    let immutable_box = Box::new(5u32);
    println!("immutable_box contains {}", immutable_box);
    // Mutability error
    //*immutable_box = 4;
    // *Move* the box, changing the ownership (and mutability)
    let mut mutable_box = immutable_box;
    println!("mutable_box contains {}", mutable_box);
    // Modify the contents of the box
    *mutable_box = 4;
    println!("mutable_box now contains {}", mutable_box);

    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` is moved out of person, but `age` is referenced
    let Person { name, ref age } = person;
    println!("The person's age is {}", age);
    println!("The person's name is {}", name);
    // Error! borrow of partially moved value: `person` partial move occurs
    //println!("The person struct is {:?}", person);
    // `person` cannot be used but `person.age` can be used as it is not moved
    println!("The person's age from person struct is {}", person.age);

    // This function takes ownership of a box and destroys it
    fn eat_box_i32(boxed_i32: Box<i32>) {
        println!("Destroying box that contains {}", boxed_i32);
    }

    // This function borrows an i32
    fn borrow_i32(borrowed_i32: &i32) {
        println!("This int is: {}", borrowed_i32);
    }
    // Create a boxed i32 in the heap, and a i32 on the stack
    // Remember: numbers can have arbitrary underscores added for readability
    // 5_i32 is the same as 5i32
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    // Borrow the contents of the box. Ownership is not taken,
    // so the contents can be borrowed again.
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);
    {
        // Take a reference to the data contained inside the box
        let _ref_to_i32: &i32 = &boxed_i32;

        // Error!
        // Can't destroy `boxed_i32` while the inner value is borrowed later in scope.
        // eat_box_i32(boxed_i32);
        // FIXME ^ Comment out this line

        // Attempt to borrow `_ref_to_i32` after inner value is destroyed
        borrow_i32(_ref_to_i32);
        // `_ref_to_i32` goes out of scope and is no longer borrowed.
    }
    // `boxed_i32` can now give up ownership to `eat_box_i32` and be destroyed
    eat_box_i32(boxed_i32);

    struct Point { x: i32, y: i32, z: i32 }
    let mut point = Point { x: 0, y: 0, z: 0 };

    let borrowed_point = &point;
    let another_borrow = &point;

    // Data can be accessed via the references and the original owner
    println!("Point has coordinates: ({}, {}, {})",
             borrowed_point.x, another_borrow.y, point.z);

    // Error! Can't borrow `point` as mutable because it's currently
    // borrowed as immutable.
    // let mutable_borrow = &mut point;
    // TODO ^ Try uncommenting this line

    // The borrowed values are used again here
    println!("Point has coordinates: ({}, {}, {})",
             borrowed_point.x, another_borrow.y, point.z);

    // The immutable references are no longer used for the rest of the code so
    // it is possible to reborrow with a mutable reference.
    let mutable_borrow = &mut point;

    // Change data via mutable reference
    mutable_borrow.x = 142857;
    mutable_borrow.y = 142857;
    mutable_borrow.z = 999999;

    // Error! Can't borrow `point` as immutable because it's currently
    // borrowed as mutable.
    // let y = &point.y;
    // TODO ^ Try uncommenting this line
    // Error! Can't print because `println!` takes an immutable reference.
    // println!("Point Z coordinate is {}", point.z);
    // TODO ^ Try uncommenting this line
    // Ok! Mutable references can be passed as immutable to `println!`
    println!("Point has coordinates: ({}, {}, {})",
             mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);

    // The mutable reference is no longer used for the rest of the code so it
    // is possible to reborrow
    let new_borrowed_point = &point;
    println!("Point now has coordinates: ({}, {}, {})",
             new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z);
    run_ref();
}

fn run_ref() {
    #[derive(Clone, Copy)]
    struct Point { x: i32, y: i32 }

    let c = 'Q';

    // A `ref` borrow on the left side of an assignment is equivalent to
    // an `&` borrow on the right side.
    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let point = Point { x: 0, y: 0 };

    // `ref` is also valid when destructuring a struct.
    let _copy_of_x = {
        // `ref_to_x` is a reference to the `x` field of `point`.
        let Point { x: ref ref_to_x, y: _ } = point;

        // Return a copy of the `x` field of `point`.
        *ref_to_x
    };

    // A mutable copy of `point`
    let mut mutable_point = point;
    {
        // `ref` can be paired with `mut` to take mutable references.
        let Point { x: _, y: ref mut mut_ref_to_y } = mutable_point;

        // Mutate the `y` field of `mutable_point` via a mutable reference.
        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!("mutable_point is ({}, {})", mutable_point.x, mutable_point.y);

    // A mutable tuple that includes a pointer
    let mut mutable_tuple = (Box::new(281457u32), 3u32);

    {
        // Destructure `mutable_tuple` to change the value of `last`.
        let (_, ref mut last) = mutable_tuple;
        *last = 142857_u32;
    }

    println!("tuple is {:?}", mutable_tuple);

    run_lifetime()
}

fn run_lifetime() {
    struct Owner(i32);
    impl Owner {
        // Annotate lifetimes as in a standalone function.
        fn add_one<'a>(&'a mut self) { self.0 += 1; }
        fn print<'a>(&'a self) {
            println!("`print`: {}", self.0);
        }
    }

    let mut owner = Owner(1142857);

    owner.add_one();
    owner.print();

    run_dyn_trait();
}

fn run_dyn_trait() {
    struct Sheep {}
    struct Cow {}

    trait Animal {
        // Instance method signature
        fn noise(&self) -> &'static str;
    }

    // Implement the `Animal` trait for `Sheep`.
    impl Animal for Sheep {
        fn noise(&self) -> &'static str {
            "Sheep baaaaah!"
        }
    }

    // Implement the `Animal` trait for `Cow`.
    impl Animal for Cow {
        fn noise(&self) -> &'static str {
            "Cow moooooo!"
        }
    }

    // Returns some struct that implements Animal, but we don't know which one at compile time.
    fn random_animal(random_number: f64) -> Box<dyn Animal> {
        if random_number < 0.5 {
            Box::new(Sheep {})
        } else {
            Box::new(Cow {})
        }
    }

    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());

    struct Droppable {
        name: &'static str,
    }

    // This trivial implementation of `drop` adds a print to console.
    impl Drop for Droppable {
        fn drop(&mut self) {
            println!("> Dropping {}", self.name);
        }
    }
    let _a = Droppable { name: "a" };

    // block A
    {
        let _b = Droppable { name: "b" };

        // block B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("Exiting block B");
        }
        println!("Just exited block B");

        println!("Exiting block A");
    }
    println!("Just exited block A");

    // Variable can be manually dropped using the `drop` function
    drop(_a);

    run_macro();
}

fn run_macro() {
    // `test!` will compare `$left` and `$right`
    // in different ways depending on how you invoke it:
    macro_rules! test_exp {
        // Arguments don't need to be separated by a comma.
        // Any template can be used!
        ($left:expr; and $right:expr) => {
            println!("{:?} and {:?} is {:?}",
                     stringify!($left),
                     stringify!($right),
                     $left && $right)
        };
        // ^ each arm must end with a semicolon.
        ($left:expr; or $right:expr) => {
            println!("{:?} or {:?} is {:?}",
                     stringify!($left),
                     stringify!($right),
                     $left || $right)
        };
    }
    test_exp!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test_exp!(true; or false);

    // 2. Macros can use + in the argument list to indicate that an argument may repeat at least once, or *, to indicate that the argument may repeat zero or more times.
    // `find_min!` will calculate the minimum of any number of arguments.
    macro_rules! find_min {
        // Base case:
        ($x:expr) => ($x);
        // `$x` followed by at least one `$y,`
        ($x:expr, $($y:expr),+) => (
            // Call `find_min!` on the tail `$y`
            std::cmp::min($x, find_min!($($y),+))
        )
    }

    println!("min: {}", find_min!(1));
    println!("min: {}", find_min!(1 + 20, 2));
    println!("min: {}", find_min!(500, 2 * 3, 4));
}
async fn manual_hello() -> impl Responder {

    // 1 do something you like
    let _immutable_binding = 1;
    let mut mutable_binding = 1;
    println!("Before mutation: {}", mutable_binding);
    // Ok
    mutable_binding += 1;
    println!("After mutation: {}", mutable_binding);

    // 2 literal
    // Suffixed literals, their types are known at initialization
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literals, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));

    // 3 type inference
    // Because of the annotation, the compiler knows that `elem` has type u8.
    let elem = 5u8;

    // Create an empty vector (a growable array).
    let mut vec = Vec::new();
    // At this point the compiler doesn't know the exact type of `vec`, it
    // just knows that it's a vector of something (`Vec<_>`).

    // Insert `elem` in the vector.
    vec.push(elem);
    // Aha! Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)
    // TODO ^ Try commenting out the `vec.push(elem)` line
    // println!("{:?}", vec);

    // 4 conversion from and to operation
    let my_str = "Rust Web Programming";
    let my_string = String::from(my_str);

    let int = 5;
    // Try removing the type annotation
    let num: Number = int.into();
    println!("我的数值是 {:?}", num);

    // 5 try_from
    // TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    let circle = Circle { radius: 142857 };

    // 6 Parsing a String
    let parsed: i32 = "142857".parse().unwrap();
    let turbo_parsed = "999999".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;

    let radius2 = "    3 ";
    let circle2: Circle = radius2.parse().unwrap();
    println!("{:?}", circle2);

    let mut magic_number = 142857u32;
    let mut loop_count = 0;
    // Infinite loop
    loop {
        loop_count += 1;
        magic_number *= loop_count;
        if magic_number == 999999 {
            println!(" 142857u32 循环 {} 等于 999999 值", loop_count);
            // Exit this loop
            break;
        } else {
            magic_number = 142857;
        }
    }

    'tree1: loop {
        println!("进入第二个循环inner");
        'tree2: loop {
            println!("业务处理逻辑循环 inner");
            break 'tree2;
        }
        println!("outer 循环逻辑");
        break 'tree1;
        // 下面代码不会被执行
    }

    let mut c_number = 142857u32;
    let loop_number = loop {
        c_number += 142857;

        if c_number == 999999 {
            break 999999 / 142857;
        }
    };
    assert_eq!(loop_number, 7);

    let mut w_n = 142857u32;
    let mut while_count = 1u32;
    // Loop while `n` is less than 101
    while w_n != 999999 {
        while_count += 1;
        // 计数器加一
        w_n += 142857;
    }
    assert_eq!(while_count, 7);

    for _n in 1..=5 {
        // 循环 5 次
        println!("for循环n值{}", _n);
    }
    let mut car_names = vec!["比亚迪", "赛里斯", "小米S7"];
    for car_name in car_names.iter_mut() {
        *car_name = match car_name {
            &mut "比亚迪" => "比亚迪品牌",
            _ => "默认品牌",
        }
    }

    let b = true;
    // Match is an expression too
    let b_number = match b {
        // The arms of a match must cover all the possible values
        false => 0,
        true => 1,
    };
    println!("match {} 对应整型数据 {}", b, b_number);
    let m_number = 11;
    match m_number {
        // Match a single value
        1 => println!("一!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("质数"),
        // Match an inclusive range
        13..=19 => println!("大于 12的整数"),
        // Handle the rest of cases
        _ => println!("默认，未匹配数据"),
    }

    // destructure
    let triple = (0, "142857", "285714");
    // Match can be used to destructure a tuple
    match triple {
        // Destructure the second and third elements
        (0, y, z) => println!("`y` = {:?}, and `z` = {:?}", y, z),
        (1, ..)  => println!("第一个是 1，其他数据不进行匹配"),
        (.., "142857")  => println!("最后一个是 `142857`,其他数据不进行匹配"),
        (3, .., "285714")  => println!("第一个是 `3`, 最后一个是 `285714`, 其他数据不进行匹配"),
        // `..` can be used to ignore the rest of the tuple
        _      => println!("未匹配到任何数据"),
        // `_` 表示不将值绑定到变量
    }

    // Try changing the values in the array, or make it a slice!
    let array = [1, 142857, 6];
    match array {
        // Binds the second and the third elements to the respective variables
        [0, second, third] => println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),
        // Single values can be ignored with _
        [1, _, third] => println!("array[0] = 1, array[2] = {} and array[1] was ignored", third),
        // You can also bind some and ignore the rest
        [-1, second, ..] => println!("array[0] = -1, array[1] = {} and all the other ones were ignored", second),
        // Or store them in another array/slice (the type depends on
        // that of the value that is being matched against)
        [3, second, tail @ ..] => println!("array[0] = 3, array[1] = {} and the other elements were {:?}", second, tail),
        // Combining these patterns, we can, for example, bind the first and
        // last values, and store the rest of them in a single array
        [first, middle @ .., last] => println!("array[0] = {}, middle = {:?}, array[2] = {}", first, middle, last),
    }

    let color = Color::Blue;
    // An `enum` can be destructured using a `match`.
    match color {
        Color::Red => println!("红色!"),
        Color::Green => println!("绿色!"),
        Color::Blue => println!("蓝色!"),
        _ => println!("default color!"),
    }

    let mut mut_v1 = 142857;
    // Use `ref mut` similarly.
    match mut_v1 {
        ref mut m => {
            // Got a reference. Gotta dereference it before we can
            // add anything to it.
            *m += 142857;
            println!("We added 142857. `mut_value`: {:?}", m);
        },
    }

    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    // Try changing the values in the struct to see what happens
    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

        // you can destructure structs and rename the variables,
        // the order is not important
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // and you can also ignore some variables:
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // this will give an error: pattern does not mention field `x`
        //Foo { y } => println!("y = {}", y),
    }

    let number: u8 = 4;

    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => unreachable!("Should never happen."),
    }
    fn get_number() -> u32 {
        17
    }
    match get_number() {
        0             => println!("I haven't celebrated my first birthday yet"),
        // Could `match` 1 ..= 12 directly but then what age
        // would the child be? Instead, bind to `n` for the
        // sequence of 1 ..= 12. Now the age can be reported.
        n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        // Nothing bound. Return the result.
        n             => println!("I'm an old person of age {:?}", n),
    }

    fn some_number() -> Option<u32> {
        Some(142857)
    }

    match some_number() {
        // Got `Some` variant, match if its value, bound to `n`,
        // is equal to 42.
        Some(n @ 42) => println!("The Answer: {}!", n),
        // Match any other number.
        Some(n)      => println!("Not interesting... {}", n),
        // Match anything else (`None` variant).
        _            => (),
    }

    // Make `optional` of type `Option<i32>`
    let mut optional = Some(0);

    // This reads: "while `let` destructures `optional` into
    // `Some(i)`, evaluate the block (`{}`). Else `break`.
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            optional = Some(i + 1);
        }
    }
    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    let one = || 1;
    println!("closure returning one: {}", one());

    let outer_var = 42;
    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inferred  = |i     |          i + outer_var  ;
    // Call the closures.
    println!("closure_annotated: {}", closure_annotated(1));
    println!("closure_inferred: {}", closure_inferred(1));

    // function capture
    capture();

    HttpResponse::Ok().body(format!("{}, {:?}, {}， radius： {}, parse string {}, {:?}",
                                    mutable_binding, vec, my_string, circle.to_string(), sum, car_names))
}

#[allow(dead_code)]
struct AppState {
    app_name: String,
    // count: Cell<usize>,
    global_count: Arc<AtomicUsize>,
    redis_conn: Mutex<Connection>,
}

//noinspection ALL
#[derive(Deserialize)]
struct Info {
    user_id: u32,
    friend: String,
}

#[derive(Deserialize)]
struct PostInfo {
    title: String,
    body: String,
}
/// extract path info using serde
#[get("/users/{user_id}/{friend}")] // <- define path parameters
async fn index_path_info(info: web::Path<Info>) -> Result<String> {
    Ok(format!("Welcome {}, user_id {}!", info.friend, info.user_id
    ))
}

#[get("/usersV2/{user_id}/{friend}")] // <- define path parameters
async fn index2(req: HttpRequest) -> Result<String> {
    let name: String = req.match_info().get("friend").unwrap().parse().unwrap();
    let userid: i32 = req.match_info().query("user_id").parse().unwrap();

    Ok(format!("Welcome {}, user_id {}!", name, userid))
}
// 获取系统名称
#[get("/index3")]
async fn index3(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("system name : {app_name}!") // <- response with app_name
}

// 获取系统名称
#[get("/index4")]
async fn index4(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    // std::thread::sleep(Duration::from_secs(5)); // <-- Bad practice! Will cause the current worker thread to hang!
    tokio::time::sleep(Duration::from_secs(5)).await; // <-- Ok. Worker thread will handle other requests here
    format!("system name : {app_name}!") // <- response with app_name
}

#[derive(Deserialize)]
struct FormData {
    username: String,
}

// URL-Encoded Forms
#[post("/indexFormData")]
async fn index_form_data(data: web::Form<FormData>) -> Result<String> {
    Ok(format!("Welcome {}!", data.username))
}

#[derive(Deserialize)]
struct UserInfo {
    user_name: String,
    password: String,
}

#[get("/getQueryParams")]
async fn get_query_params(info: web::Query<UserInfo>) -> String {
    format!("Welcome {}! , password: {}!", info.user_name, info.password)
}

#[post("/submitForm")]
async fn submit_form(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Welcome {}, {}", info.user_id, info.friend))
}

#[derive(Serialize)]
struct MyObj {
    name: &'static str,
    age: u32,
}

#[allow(dead_code)]
#[derive(Serialize)]
struct MyObj2<T> {
    name: &'static str,
    age: u32,
    data: T,
}

// Responder
impl Responder for MyObj {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

async fn index_body_json() -> impl Responder {
    MyObj { name: "user", age: 28}
}

#[get("/stream_gen")]
async fn stream_gen() -> HttpResponse {
    let txt_stream = b"Response body can be generated asynchronously. In this case, body must implement the stream trait Stream<Item = Result<Bytes, Error>>";
    let body = once(ok::<_, actix_web::Error>(web::Bytes::from_static(txt_stream)));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}


#[derive(Debug, Display, Error)]
#[display("my error: {name}")]
pub struct MyError {
    name: &'static str,
}

// Use default implementation for `error_response()` method
impl error::ResponseError for MyError {}

#[get("/index_for_err")]
async fn index_for_err() -> Result<&'static str, MyError> {
    let err = MyError { name: "test error" };
    info!("{}", err);
    Err(err)
}
// resource route location request
// this function could be located in a different module
fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/gapi/test1")
            .route(web::get().to(|| async { HttpResponse::Ok().body("test1") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

// #[get("/get_count")]
// async fn get_count(data: web::Data<AppState>) -> impl Responder {
//     format!("count: {}, global count {}", data.count.get(), data.global_count.load(Ordering::Relaxed))
// }
//
// #[post("/increase_one")]
// async fn increase_one(data: web::Data<AppState>) -> impl Responder {
//     let count = data.count.get();
//     data.global_count.fetch_add(1, Ordering::Relaxed);
//     data.count.set(count + 1);
//
//     format!("count: {}, global count {}", data.count.get(), data.global_count.load(Ordering::Relaxed))
// }


type RegisterResult = actix_web::Either<HttpResponse, Result<&'static str, actix_web::Error>>;

async fn index_two_body_type(info: web::Query<UserInfo>) -> RegisterResult {
    if info.user_name == "admin" {
        // choose Left variant
        actix_web::Either::Left(HttpResponse::BadRequest().body("Bad data"))
    } else {
        // choose Right variant
        actix_web::Either::Right(Ok("Hello!"))
    }
}


#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .insert_header(("X-A", "test"))
        .body("data")
}

async fn stream_sse(_req: HttpRequest) -> HttpResponse {
    let mut counter: usize = 5;

    // yields `data: N` where N in [5; 1]
    let server_events =
        futures::stream::poll_fn(move |_cx| -> Poll<Option<Result<web::Bytes, actix_web::Error>>> {
            if counter == 0 {
                return Poll::Ready(None);
            }
            let payload = format!("data: {}\n\n", counter);
            counter -= 1;
            Poll::Ready(Some(Ok(web::Bytes::from(payload))))
        });

    HttpResponse::build(StatusCode::OK)
        .insert_header((http::header::CONTENT_TYPE, "text/event-stream"))
        .insert_header(ContentEncoding::Identity)
        .streaming(server_events)
}

#[allow(dead_code)]
async fn index_plaintext_call() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .insert_header(("X-Hdr", "sample"))
        .body("data")
}


async fn my_x_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    info!("my_x_middleware before> > > {}", req.path());
    let res = next.call(req).await;
    // post-processing
    info!("my_x_middleware after > > >");
    res
}

fn add_error_header<B>(mut res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    error_log!(">>>  error_header ");
    res.response_mut().headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("Error"),
    );

    Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}


async fn echo(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let mut stream = stream
                                    .aggregate_continuations()
                                    // aggregate continuation frames up to 1MiB
                                    .max_continuation_size(2_usize.pow(20));

    // start task but don't wait for it
    rt::spawn(async move {
        // receive messages from websocket
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(AggregatedMessage::Text(text)) => {
                    // echo text message
                    session.text(text).await.unwrap();
                }

                Ok(AggregatedMessage::Binary(bin)) => {
                    // echo binary message
                    session.binary(bin).await.unwrap();
                }

                Ok(AggregatedMessage::Ping(msg)) => {
                    // respond to PING frame with PONG frame
                    session.pong(&msg).await.unwrap();
                }

                _ => {}
            }
        }
    });

    // respond immediately with response connected to WS session
    Ok(res)
}

use diesel::prelude::*;
use self::models::*;
use dotenvy::dotenv;
use std::env;
#[allow(unused_imports)]
use diesel::r2d2::{self, ConnectionManager, Error};
use crate::common::model::AR;
// simple type
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn establish_db_connection() -> r2d2::Pool<ConnectionManager<PgConnection>> {

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");
    return pool;
}


#[post("/post/createPosts")]
async fn create_posts(pool: web::Data<DbPool>, info: web::Json<PostInfo>) -> actix_web::Result<impl Responder> {
    use crate::schema::r_posts;
    let title = info.title.clone();
    let body = info.body.clone();
    let mut conn = pool.get().expect("Failed to get DB connection");

    let new_post = NewPost{ title: title.as_str(), body: body.as_str()};

    let result = diesel::insert_into(r_posts::table)
                        .values(&new_post)
                        .get_result::<models::RPosts>(&mut conn);

    match result {
        Ok(data) => Ok(HttpResponse::Ok().json(AR::success(Some(data)))),
        Err(r) =>  Ok(HttpResponse::Ok().json(AR::<()>::error(504, Some(r.to_string())))),
    }
}

async fn index_json_diesel(pool: web::Data<DbPool>) -> Result<impl Responder> {
    use self::schema::r_posts::dsl::*;

    // TODO ... pagination
    let mut con = pool.get().expect("Failed to get DB connection");
    let results = r_posts
        .filter(published.eq(true))
        .limit(5)
        .select(RPosts::as_select())
        .load(&mut con)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    if results.len() > 0 {
        Ok(HttpResponse::Ok().json(AR::success_flag(Some(results), true)))
    } else {
        Ok(HttpResponse::Ok().json(AR::<Vec<RPosts>>::success(Some(Vec::with_capacity(0)))))
    }
}

#[allow(dead_code)]
async fn update_posts(req: HttpRequest, pool: web::Data<DbPool>) -> Result<impl Responder>  {

    use self::schema::r_posts::dsl::*;
    let id_str: String = req.match_info().get("id").unwrap().parse().unwrap();
    let number_id: i32 = id_str.parse().unwrap();
    let mut con = pool.get().expect("Failed to get DB connection");
    let post = diesel::update(r_posts.find(number_id))
        .set(published.eq(true))
        .returning(RPosts::as_returning())
        .get_result::<RPosts>(&mut con)
        .unwrap();
    Ok(HttpResponse::Ok().json(AR::success(Some(post))))
}


#[allow(dead_code)]
async fn get_post_by_id(req: HttpRequest, pool: web::Data<DbPool>) -> Result<impl Responder>  {

    use self::schema::r_posts::dsl::*;
    let id_str: String = req.match_info().get("id").unwrap().parse().unwrap();
    let number_id: i32 = id_str.parse().unwrap();
    let mut con = pool.get().expect("Failed to get DB connection");

    let post = r_posts
                    .find(number_id)
                    .select(RPosts::as_select())
                    .first(&mut con)
                    .optional();

    match post {
        Ok(Some(post)) => Ok(HttpResponse::Ok().json(AR::success(Some(post)))),
        Ok(None) => Ok(HttpResponse::Ok().json(AR::success(Some(())))),
        Err(r) => Ok(HttpResponse::Ok().json(AR::<String>::error(503, Some(r.to_string()))))
    }
}

#[allow(dead_code)]
async fn delete_posts(req: HttpRequest, pool: web::Data<DbPool>) -> Result<impl Responder> {
    use self::schema::r_posts::dsl::*;

    let id_str: String = req.match_info().get("id").unwrap().parse().unwrap();
    let number_id: i32 = id_str.parse().unwrap();

    let mut con = pool.get().expect("Failed to get DB connection");
    let num_deleted = diesel::delete(r_posts.find(number_id))
        .execute(&mut con)
        .expect("Error deleting posts");
    if num_deleted > 0 {
        Ok(HttpResponse::Ok().json(AR::success(Some(()))))
    } else {
        Ok(HttpResponse::Ok().json(AR::success_flag(Some(()), false)))
    }
}

// Data model
#[derive(Debug, Serialize, Deserialize)]
struct Item {
    id: String,
    name: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ItemKV {
    key: String,
    value: Item,
}

// Create operation
async fn create_redis_item(
    state: web::Data<AppState>,
    item: web::Json<Item>,
) -> impl Responder {
    let mut conn = state.redis_conn.lock().unwrap();

    let key = format!("item:{}", item.id);
    let value = serde_json::to_string(&item).unwrap();

    match conn.set::<String, String, String>(key, value) {
        Ok(_) => HttpResponse::Ok().json("Item created successfully"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// Read operation
async fn get_redis_item(state: web::Data<AppState>, id: web::Path<String>, ) -> impl Responder {
    let mut conn = state.redis_conn.lock().unwrap();
    let key = format!("item:{}", id);
    match conn.get::<String, String>(key) {
        Ok(value) => match serde_json::from_str::<Item>(&value) {
            Ok(item) => HttpResponse::Ok().json(item),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        },
        Err(_) => HttpResponse::NotFound().body("Item not found"),
    }
}

// List all items (using pattern matching)
async fn load_redis_items(state: web::Data<AppState>) -> impl Responder {
    let mut conn = state.redis_conn.lock().unwrap();

    match conn.keys::<String, Vec<String>>("item:*".to_string()) {
        Ok(keys) => {
            let mut items = Vec::with_capacity(keys.len());
            
            for key in keys {
                // Use match for better error handling
                match conn.get::<String, String>(key.clone()) {
                    Ok(value) => {
                        match serde_json::from_str::<Item>(&value) {
                            Ok(item) => {
                                items.push(ItemKV {
                                    key,
                                    value: item
                                });
                            },
                            Err(e) => {
                                error_log!("Failed to parse item from Redis value: {}", e);
                                continue;
                            }
                        }
                    },
                    Err(e) => {
                        error_log!("Failed to get value for key {}: {}", key, e);
                        continue;
                    }
                }
            }
            HttpResponse::Ok().json(AR::success(Some(items)))
        }
        Err(e) => {
            error_log!("Failed to get keys from Redis: {}", e);
            HttpResponse::InternalServerError().json(AR::<Vec<ItemKV>>::error(500, Some(e.to_string())))
        }
    }
}

async fn delete_item(state: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
    let mut conn = state.redis_conn.lock().unwrap();

    let key = format!("item:{}", id);
    match conn.del::<String, i32>(key) {
        Ok(1) => HttpResponse::Ok().json("Item deleted successfully"),
        Ok(0) => HttpResponse::NotFound().body("Item not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        _ => HttpResponse::InternalServerError().body("Unexpected error"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Actix logs all errors at the WARN log level.
    unsafe { std::env::set_var("RUST_LOG", "info"); }
    unsafe { std::env::set_var("RUST_BACKTRACE", "1"); }
    env_logger::init();

    // redis cache : Connect to Redis
    dotenv().ok();
    let redis_url = env::var("REDIS_URL").expect("DATABASE_URL must be set");
    let client = Client::open(redis_url).unwrap();
    let conn = client.get_connection().unwrap();

    // db pool
    let pool = establish_db_connection();
    // redis connection
    let data = web::Data::new(AppState {
        app_name: String::from("FastlyActix"),
        global_count: Arc::new(AtomicUsize::new(0)),
        redis_conn: Mutex::new(conn),
    });

    HttpServer::new(move || {
        let logger = Logger::default();
        let json_config = web::JsonConfig::default()
            // 4K * 225
            .limit(4096*225)
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                    .into()
            });

        App::new()
            .wrap(logger)
            .wrap(Compress::default())
            // .wrap_fn(|req, srv| {
            //     info!(">>>. You requested: {}", req.path());
            //     srv.call(req).map(|res| {
            //         info!(">>>. You response ");
            //         res
            //     })
            // })
            .wrap(from_fn(my_x_middleware))
            .wrap(
                ErrorHandlers::new()
                    .handler(StatusCode::INTERNAL_SERVER_ERROR, add_error_header),
            )
            .configure(config)
            .service(
                web::scope("/gapi")
                    .app_data(json_config)
                    .app_data(data.clone())
                    .app_data(web::Data::new(pool.clone()))
                    .route("/hi", web::get().to(manual_hello))
                    .route("/index2", web::get().to(manual_hello))
                    .route("/indexBodyJson", web::get().to(index_body_json))
                    .route("/indexTwoBody", web::get().to(index_two_body_type))
                    .route("/stream_sse", web::get().to(stream_sse))
                    .route("/echo", web::get().to(echo))
                    .route("/post/loadPosts", web::get().to(index_json_diesel))
                    .route("/post/updatePost/{id}", web::post().to(update_posts))
                    .route("/post/getPostById/{id}", web::get().to(get_post_by_id))
                    .route("/post/removePost/{id}", web::post().to(delete_posts))
                    .route("/cache/createCacheItem", web::post().to(create_redis_item))
                    .route("/cache/getCacheItem/{id}", web::get().to(get_redis_item))
                    .route("/cache/loadRedisItems", web::get().to(load_redis_items))
                    .route("/cache/deleteItem/{id}", web::post().to(delete_item))
                    .service(web::resource("/error-not-found").route(web::get().to(HttpResponse::InternalServerError)))
                    .service(submit_form)
                    .service(index_path_info)
                    .service(index2)
                    .service(index3)
                    .service(index4)
                    // .service(get_count)
                    // .service(increase_one)
                    .service(index_form_data)
                    .service(get_query_params)
                    .service(stream_gen)
                    .service(index_for_err)
                    .service(create_posts)

            )

    })
       // .workers(4)  // by default this number is equal to the number of physical CPUs in the system
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
// resources can only have one owner

#[cfg(test)]
mod tests {
    use actix_web::{http::header::ContentType, test, App};
    use actix_web::{body, body::MessageBody as _, rt::pin, web};
    use super::*;

    #[actix_web::test]
    async fn test_index_get() {
        let app = test::init_service(App::new().service(index)).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_index_post() {
        let app = test::init_service(App::new().service(index)).await;
        let req = test::TestRequest::post().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }


    use futures::future;

    #[actix_web::test]
    async fn test_stream_chunk() {
        let app = test::init_service(App::new().route("/gapi/stream_sse", web::get().to(stream_sse))).await;
        let req = test::TestRequest::get().to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body = resp.into_body();
        pin!(body);

        // first chunk
        let bytes = future::poll_fn(|cx| body.as_mut().poll_next(cx)).await;
        assert_eq!(
            bytes.unwrap().unwrap(),
            web::Bytes::from_static(b"data: 5\n\n")
        );

        // second chunk
        let bytes = future::poll_fn(|cx| body.as_mut().poll_next(cx)).await;
        assert_eq!(
            bytes.unwrap().unwrap(),
            web::Bytes::from_static(b"data: 4\n\n")
        );

        // remaining part
        for i in 0..3 {
            let expected_data = format!("data: {}\n\n", 3 - i);
            let bytes = future::poll_fn(|cx| body.as_mut().poll_next(cx)).await;
            assert_eq!(bytes.unwrap().unwrap(), web::Bytes::from(expected_data));
        }
    }

    #[actix_web::test]
    async fn test_stream_full_payload() {
        let app = test::init_service(App::new().route("/gapi/stream_sse", web::get().to(stream_sse))).await;
        let req = test::TestRequest::get().to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body = resp.into_body();
        let bytes = body::to_bytes(body).await;
        assert_eq!(
            bytes.unwrap(),
            web::Bytes::from_static(b"data: 5\n\ndata: 4\n\ndata: 3\n\ndata: 2\n\ndata: 1\n\n")
        );
    }

    // just test method calling
    #[actix_web::test]
    async fn test_index_ok() {
        // let req = test::TestRequest::default()
        //     .insert_header(ContentType::plaintext())
        //     .to_http_request();
        let resp = index_plaintext_call().await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    // #[actix_web::test]
    // async fn fetch_an_integer() {
    //     let nodes = vec!["redis://10.111.27.32:6379/11"];
    //     let client = redis::cluster::ClusterClient::new(nodes).unwrap();
    //     let mut connection = client.get_connection().unwrap();
    //     let _: () = connection.set("test", "test_data").unwrap();
    //     let rv: String = connection.get("test").unwrap();
    //     println!("{}", rv)
    // }

}

