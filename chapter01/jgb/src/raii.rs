use std::{ thread};
use std::collections::HashMap;
use std::io::Write;


// raii.rs
#[allow(dead_code)]
pub fn create_box() {
    // Allocate an integer on the heap
    let _box1 = Box::new(3i32);

    // `_box1` is destroyed here, and memory gets freed
}

#[allow(dead_code)]
pub fn do_thread() {
    let mut children_threads = vec![];

    let count = 5;
    for i in 0..count {
        children_threads.push(thread::spawn(move || {
            println!(">>> thread i {}", i)
        }))
    }
    for child_thread in children_threads {
        let _ = child_thread.join();
    }
}

/// map reduce thread scheduling
#[allow(dead_code)]
pub fn do_map_reduce() {
    let data_number = "1234567890
    12345678901234567890
    123456789012345678901234567890
    1234567890123456789012345678901234567890
    12345678901234567890123456789012345678901234567890
    123456789012345678901234567890123456789012345678901234567890
    1234567890123456789012345678901234567890123456789012345678901234567890
    12345678901234567890123456789012345678901234567890123456789012345678901234567890
    123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890
    1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890
    12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890
    123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890
    1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890
    12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890";

    let mut reduce_vector = vec![];

    let children_numbers = data_number.split_ascii_whitespace();
    for (i, line_number) in children_numbers.enumerate() {
        reduce_vector.push(thread::spawn(move || -> u32 {
            let result = line_number.chars().map(|c| c.to_digit(10).expect("expect a number value")).sum();
            println!(">>> line index {}, result : {}", i, result);
            return result
        }))
    }

    let final_result :u32 = reduce_vector.into_iter().map(|s| s.join().unwrap()).sum::<u32>();

    println!(">>> final sum result: {}", final_result)
}

#[allow(dead_code)]
pub fn do_channel() {
    use std::sync::mpsc::{Sender, Receiver};
    use std::sync::mpsc;
    use std::thread;

    static NTHREADS: i32 = 3;
    // Channels have two endpoints: the `Sender<T>` and the `Receiver<T>`,
    // where `T` is the type of the message to be transferred
    // (type annotation is superfluous)
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let mut children = Vec::new();

    for id in 0..NTHREADS {
        // The sender endpoint can be copied
        let thread_tx = tx.clone();

        // Each thread will send its id via the channel
        let child = thread::spawn(move || {
            // The thread takes ownership over `thread_tx`
            // Each thread queues a message in the channel
            thread_tx.send(id).unwrap();

            // Sending is a non-blocking operation, the thread will continue
            // immediately after sending its message
            println!("thread {} finished", id);
        });

        children.push(child);
    }

    // Here, all the messages are collected
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        // The `recv` method picks a message from the channel
        // `recv` will block the current thread if there are no messages available
        ids.push(rx.recv());
    }

    // Wait for the threads to complete any remaining work
    for child in children {
        child.join().expect("oops! the child thread panicked");
    }

    // Show the order in which the messages were sent
    println!("{:?}", ids);
}

#[allow(dead_code)]
pub fn do_path_ops() {
    use std::path::Path;

    // Create a `Path` from an `&'static str`
    let path = Path::new(".");

    // The `display` method returns a `Display`able structure
    let _display = path.display();

    // `join` merges a path with a byte container using the OS specific
    // separator, and returns a `PathBuf`
    let mut new_path = path.join("a").join("b");

    // `push` extends the `PathBuf` with a `&Path`
    new_path.push("c");
    new_path.push("myfile.tar.gz");

    // `set_file_name` updates the file name of the `PathBuf`
    new_path.set_file_name("package.tgz");

    // Convert the `PathBuf` into a string slice
    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }
}

#[allow(dead_code)]
pub fn do_read_file() {
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    // Create a path to the desired file
    let path = Path::new("config.yaml");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
}

#[allow(dead_code)]
pub fn do_create_file() {
    static LOREM_IPSUM: &str =
        "foo: bar
         pleh: help
         stuff:
           foo: bar
           bar: foo";

    // upgrade version use std::fs::OpenOptions;
    use std::fs::OpenOptions;
    let mut file = OpenOptions::new().write(true).open("_config.txt").unwrap();

    // use std::path::Path;
    // use std::fs::File;
    // let file_path = Path::new("_config.txt");
    // let display_file = file_path.display();
    //
    // let mut file = match File::create(file_path) {
    //     Err(e) => panic!(" can not open file {}, error: {}", display_file, e),
    //     Ok(file) => file,
    // };

    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(e) => panic!(" write to file with error {}", e),
        Ok(_) => println!(" write to file successfully!")
    }
}

#[allow(dead_code)]
pub fn do_read_file_lines() {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;
    // The output is wrapped in a Result to allow matching on errors.
    // Returns an Iterator to the Reader of the lines of the file.
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./hosts.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            println!("{}", line);
        }
    }
}

#[allow(dead_code)]
pub fn do_cmd() {
    use std::process::Command;

    let output = Command::new("rustc")
        .arg("--version")
        .output().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);

        print!("rustc succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);

        print!("rustc failed and stderr was:\n{}", s);
    }
}

#[allow(dead_code)]
pub fn do_ffi() {
    use std::fmt;

    // this extern block links to the libm library
    #[cfg(target_family = "windows")]
    #[link(name = "msvcrt")]
    extern {
        // this is a foreign function
        // that computes the square root of a single precision complex number
        fn csqrtf(z: Complex) -> Complex;

        fn ccosf(z: Complex) -> Complex;
    }
    #[cfg(target_family = "unix")]
    #[link(name = "m")]
    extern {
        // this is a foreign function
        // that computes the square root of a single precision complex number
        fn csqrtf(z: Complex) -> Complex;

        fn ccosf(z: Complex) -> Complex;
    }

    // Since calling foreign functions is considered unsafe,
    // it's common to write safe wrappers around them.
    fn cos(z: Complex) -> Complex {
        unsafe { ccosf(z) }
    }

    // Minimal implementation of single precision complex numbers
    #[repr(C)]
    #[derive(Clone, Copy)]
    struct Complex {
        re: f32,
        im: f32,
    }

    impl fmt::Debug for Complex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if self.im < 0. {
                write!(f, "{}-{}i", self.re, -self.im)
            } else {
                write!(f, "{}+{}i", self.re, self.im)
            }
        }
    }

    // z = -1 + 0i
    let z = Complex { re: -1., im: 0. };

    // calling a foreign function is an unsafe operation
    let z_sqrt = unsafe { csqrtf(z) };

    println!("the square root of {:?} is {:?}", z, z_sqrt);

    // calling safe API wrapped around unsafe operation
    println!("cos({:?}) = {:?}", z, cos(z));
}

#[allow(dead_code)]
pub fn do_sqrt(f : f64) -> Result<f64, String> {
    if f > 0.0 {
        Ok(f.powf(0.5))
    } else {
        Err("can not handle negative number {f}".to_owned())
    }
}

#[allow(dead_code)]
pub fn do_unsafe() {
    let raw_p: *const u32 = &10;

    unsafe {
        assert!(*raw_p == 10);
    }
}

#[allow(dead_code)]
pub fn do_string() {
    let tiny_str: &'static str = "&str is a slice (&[u8]) that always points to a valid UTF-8 sequence";
    for word in tiny_str.split_ascii_whitespace().rev() {
        println!("word {}", word)
    }

    // heap String
    let one_string = String::from("It is my book");
    let two_string = one_string.replace("book", "pencil");
    println!(">>> one_string {}, two_string: {}", one_string, two_string);

    // Copy chars into a vector, sort and remove duplicates
    let mut chars: Vec<char> = tiny_str.chars().collect();
    chars.sort();
    chars.dedup();

    // Create an empty and growable `String`
    let mut string = String::new();
    for c in chars {
        // Insert a char at the end of string
        string.push(c);
        // Insert a string at the end of string
        string.push_str(", ");
    }

    // The trimmed string is a slice to the original string, hence no new
    // allocation is performed
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);

    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);
    // If you need "# in your string, just use more #s in the delimiter.
    // You can use up to 255 #s.
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);

    let escaped = b"\x52\x75\x73\x74 as bytes";
    // ...but no unicode escapes
    // let escaped = b"\u{211D} is not allowed";
    println!("Some escaped bytes: {:?}", escaped);
    let raw_byte_string = br"\u{211D} is not escaped here";
    println!("{:?}", raw_byte_string);
}


#[allow(dead_code)]
pub fn do_hash_map() {
    fn call(number: &str) -> &str {
        match number {
            "798-1364" => "We're sorry, the call cannot be completed as dialed.
            Please hang up and try again.",
            "645-7689" => "Hello, this is Mr. Awesome's Pizza. My name is Fred.
            What can I get for you today?",
            _ => "Hi! Who is this again?"
        }
    }

    let mut str_map = HashMap::new();
    str_map.insert("Daniel", "798-1364");
    str_map.insert("Daniel2", "798-13642");
    str_map.insert("Ashley", "798-13643");

    // Takes a reference and returns Option<&V>
    match str_map.get(&"Daniel") {
        Some(&number) => println!("Calling Daniel: {}", call(number)),
        _ => println!("Don't have Daniel's number."),
    }

    // `HashMap::insert()` returns `None`
    // if the inserted value is new, `Some(value)` otherwise
    str_map.insert("Daniel", "164-6743");

    match str_map.get(&"Ashley") {
        Some(&number) => println!("Calling Ashley: {}", call(number)),
        _ => println!("Don't have Ashley's number."),
    }

    str_map.remove(&"Ashley");

    // `HashMap::iter()` returns an iterator that yields
    // (&'a key, &'a value) pairs in arbitrary order.
    for (contact, &number) in str_map.iter() {
        println!("Calling {}: {}", contact, call(number));
    }
}

pub fn do_hash_set() {
    use std::collections::HashSet;
    let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = vec![2i32, 3, 4].into_iter().collect();

    assert!(a.insert(4));
    assert!(a.contains(&4));

    // `HashSet::insert()` returns false if
    // there was a value already present.
    assert!(!b.insert(4), "Value 4 is already in set B!");

    b.insert(5);

    // If a collection's element type implements `Debug`,
    // then the collection implements `Debug`.
    // It usually prints its elements in the format `[elem1, elem2, ...]`
    println!("A: {:?}", a);
    println!("B: {:?}", b);

    // Print [1, 2, 3, 4, 5] in arbitrary order
    println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());

    // This should print [1]
    println!("Difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());

    // Print [2, 3, 4] in arbitrary order.
    println!("Intersection: {:?}", a.intersection(&b).collect::<Vec<&i32>>());

    // Print [1, 5]
    println!("Symmetric Difference: {:?}",
             a.symmetric_difference(&b).collect::<Vec<&i32>>());

    use std::rc::Rc;

    let rc_examples = "Rc examples".to_string();
    {
        println!("--- rc_a is created ---");

        let rc_a: Rc<String> = Rc::new(rc_examples);
        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

        {
            println!("--- rc_a is cloned to rc_b ---");

            let rc_b: Rc<String> = Rc::clone(&rc_a);
            println!("Reference Count of rc_b: {}", Rc::strong_count(&rc_b));
            println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

            // Two `Rc`s are equal if their inner values are equal
            println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));

            // We can use methods of a value directly
            println!("Length of the value inside rc_a: {}", rc_a.len());
            println!("Value of rc_b: {}", rc_b);

            println!("--- rc_b is dropped out of scope ---");
        }

        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

        println!("--- rc_a is dropped out of scope ---");
    }
    // Error! `rc_examples` already moved into `rc_a`
    // And when `rc_a` is dropped, `rc_examples` is dropped together
    // println!("rc_examples: {:?}", rc_examples);
}

use lazy_static::lazy_static;
lazy_static! {
    static ref GLOBAL_VAR_1: i32 = 142857;
}

use std::sync::OnceLock;

fn hashmap_food() -> &'static HashMap<u32, &'static str> {
    static HASHMAP: OnceLock<HashMap<u32, &str>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert(0, "banana");
        m.insert(1, "pear");
        m.insert(2, "orange");
        m
    })
}
fn do_variable() {
    // 1. 不可变变量 Immutable Variables
    let a = 5; // Immutable integer
    let b = "hello"; // Immutable string

    // 2. 可变变量
    let mut counter = 0; // Mutable integer
    counter += 1; // Increment the counter

    // 3. 类型注解 Type Annotation
    let a1: i32 = 142857; // Integer with explicit type annotation
    let b1: char = 'A'; // Character with explicit type annotation

    // 4. 类型引用 Type Inference
    let message = "Hello, Rust!"; // Type inferred as string slice(&str)
    // 5. Shadowing You can shadow a variable by reusing the let keyword.
    let c = 142857;
    let c = c + 142857; // Shadowing the variable

    // 5. 常亮 Constants: Constants are declared using the const keyword.
    const PI: f32 = 3.14; // Constant with type annotation
    // Global Variables: Rust doesn't have true global variables,
    // but you can use the lazy_static crate for similar behavior. 注意这里是 crate 实现
    // use lazy_static::lazy_static;
    // lazy_static! {
    //     static ref GLOBAL_VAR: i32 = 42;
    // }
    //
    // println(">>> global variables: {}", GLOBAL_VAR);
    // Global variables using lazy_static
    println!("GLOBAL_VAR_1 : {}", *GLOBAL_VAR_1);
    // best case
    // First access to `HASHMAP` initializes it
    println!("The entry for `0` is \"{}\".", hashmap_food().get(&0).unwrap());

    // Any further access to `HASHMAP` just returns the computed value
    println!("The entry for `1` is \"{}\".", hashmap_food().get(&1).unwrap());

    // ----------primitive data type---------
    // 1. integer
    // Signed integers: i8, i16, i32, i64, i128
    // Unsigned integers: u8, u16, u32, u64, u128
    let pi: f64 = 3.14;
    // 2. bool: Represents either true or false
    let is_true: bool = true;
    // 3. Characters: 字符类型
    let char_a: char = 'A';
    // 4. Tuples: A group of values of different types enclosed in parentheses.
    let t_multiple: (i32, f64, char) = (42, 3.14, 'A');
    // 5. Arrays
    let x_array: [i32; 3] = [1, 2, 3];
    // 6. Slices 切片  A view into a contiguous sequence of elements in a
    // collection
    let array = [1, 2, 3, 4, 5];
    let slice: &[i32] = &array[1..4]; // slice of [2, 3, 4]
    // 7. Strings： A collection of characters
    let string: String = String::from("Hello, Rust!");
    // 8. Unit Type: Represents an empty tuple ()
    let unit: () = ();
    // 9. Raw Pointers
    // *const T: Immutable raw pointer
    // *mut T: Mutable raw pointer
    let data: i32 = 42;
    let raw_ptr: *const i32 = &data;

    // ------------组合数据类型 Tuples, Arrays, Structs------------------
    // 1. Tuples: A tuple is an ordered collection of elements of
    // different types, enclosed in parentheses.
    let person: (String, usize, char) = ("Alice".to_string(), 30, 'A');

    // 2. Arrays: An array is a fixed-size, contiguous collection of
    // elements of the same type.
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    // Accessing array elements:
    let first_element = numbers[0];
    let second_element = numbers[1];
    // 2.1 Slices: A slice is a view into a contiguous sequence of
    // elements in a collection.

    // 3. Structs:
    // A struct is a way to define a custom data type by grouping
    // together values of different types.
    struct Person {
        name: String,
        age: usize,
        grade: char,
    }
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        grade: 'A',
    };
    // 3.1 Option and Result Enums: Option and Result are
    // standard enums for representing optional and error-prone
    // values.
    let some_value: Option<i32> = Some(42);
    let result_value: Result<i32, &str> = Ok(42);
    // Using match to handle Option:
    match some_value {
        Some(value) => println!("Got a value: {}", value),
        None => println!("Got None"),
    }

    // flow statement
    do_flow();
}

fn do_flow() {
    // 1. match case
    // Pattern matching in Rust allows you to destructure and
    // match on the structure of values, providing a concise and
    // powerful way to handle various cases.
    let x_day = "Monday";
    match x_day  {
        "Monday" => println!("It is the start of week!"),
        "Sunday" => println!("It is the end of week!"),
        d_ => println!("It is the default day!")
    }

    fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
        if b == 0 {
            return Err("b can not be zero!")
        }
        Ok(a/b)
    }

    match divide(100, 5) {
        Ok(t) => println!(">>> 100/5 = {}", t),
        Err(e) => println!("error info {}", e)
    }
    match divide(100, 0) {
        Ok(t) => println!(">>> 100/5 = {}", t),
        Err(e) => println!(">>> 100/0, error info {}", e)
    }

    fn find_num_element(arr: &[i32], e: i32) -> Option<usize> {
        for (index, &value) in arr.into_iter().enumerate() {
            if (value == e) {
               return Some(index);
            }
        }
        None
    }

    let numbers = [20, 40, 69, 80, 100];
    match find_num_element(&numbers, 100) {
        Some(x) => println!(">>> find one in array index {}", x),
        None => println!(">>>None element is found!")
    }
    match find_num_element(&numbers, 21) {
        Some(index) => println!("Element {} found at index{}", 21, index),
        None => println!("Element {} not found", 21),
    }

    // generic and trait
    do_generic();
    do_trait();
}

fn do_generic() {
    fn find_max<T>(a: T, b: T)  -> T where T: PartialOrd, {
        if (a > b) {
            a
        } else {
            b
        }
    }

    println!(">>> 1 compare 10, max value is {}", find_max(1, 10));
    println!(">>> 10 compare 10, max value is {}", find_max(1, 10));
    println!(">>> 10 compare 11, max value is {}", find_max(10, 11));

    println!(">>> 10.09 compare 10.08, max value is {}", find_max(10.09, 10.08));
}

fn do_trait() {
    trait Printable {
       fn print(&self);
    }

    struct Point {
        x: f64,
        y: f64,
    }
    impl Printable for Point {
        fn print(&self) {
            println!(">>> x:{}, y: {}", self.x, self.y);
        }
    }
    let p = Point{x: 100.01, y: 100.2};
    p.print();

    fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
        if s1.len() > s2.len() {
            s1
        } else {
            s2
        }
    }

    let s1 = String::from("test1");
    let s2 = String::from("test11");
    let result;
    {
        // Creating references with explicit lifetimes
        let r1 = &s1;
        let r2 = &s2;
        // Calling a function with references and explicit lifetime
        result = longest(r1, r2);
    }
    // Using the result outside the inner scope
    println!("The longest string is: {}", result);
}
/// 1. cmd : cargo test
/// 2. specify test case: cargo test test_do_sqrt∂
/// 3. Tests can be marked with the #[ignore] attribute to exclude some tests. cmd:
/// only test ignore case: cargo test -- --ignored
/// skip ignore test : cargo test
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq; // crate for test-only use

    #[test]
    fn test_run_raii() {
        // Creating lots of boxes just for fun
        // There's no need to manually free memory!
        for _ in 0u32..1_000 {
            create_box();
        }

        do_thread();
        do_map_reduce();
        do_channel();
        do_path_ops();
        do_read_file();
        do_create_file();
        do_read_file_lines();
        do_cmd();
        do_ffi();
        do_unsafe();
        use std::arch::asm;

        unsafe {
            asm!("nop");
        }

        do_string();
        do_hash_map();
        do_hash_set();
    }

    #[test]
    #[ignore]
    fn test_do_sqrt() -> Result<(), Box<dyn std::error::Error>>{
        let x = 4.0;
        assert_eq!(do_sqrt(x)?.powf(2.0), x);
        Ok(())
    }

    #[test]
    fn test_ownership_relation() {
        // 1. Ownership of a value can be transferred to a new
        // variable using the let keyword
        let s1 = String::from("Welcome to Rust language word!");
        let s2 = s1; // s1's ownership is transferred to s2
        println!("ownership s2: {}", s2);

        // 2. Some types implement the Copy
        // trait, allowing them to be copied rather than transferred.
        let n = 142857;
        let m = n;
        println!(">>> m copy value: {}", m);

        fn calculate_length(s: &String) -> usize {
            s.len()
        }
        // 3. Borrowing
        //  Instead of transferring ownership, variables
        // can borrow references to values.
        let str1 = String::from("G1");
        let len = calculate_length(&str1); // "&" creates a reference to s1
        println!(">>> str1 len : {}", len);
    }

    #[test]
    fn test_basic_variable() {
        do_variable();
    }
}
