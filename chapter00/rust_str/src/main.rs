

fn call_common() {
    let a1 = "Rust";
    let a2 = "programming";
    // 宏拼接策略
    let a = format!("{} {}", a1, a2);
    println!("{}", a); // Output: "Rust programming"

    let b1 = "Rust";
    let b2 = "programming";

    // Using push_str method for concatenation
    let mut b = b1.to_string();
    b.push_str(" ");
    b.push_str(b2);
    println!("{}", b); // Output: "Rust programming"

    let c1 = "Rust";
    let c2 = "programming";
    let vec = vec![c1, c2];
    let c = vec.join(" ");
    println!("{}", c);

    const D: &str = concat!("Rust", " ", "programming");
    println!("{}", D);

    let e1 = "Rust";
    let e2 = "programming";

    let e = e1.to_string() + " " + e2;
    println!("{}", e); // Output: "Rust programming"
}

fn do_match() {
    let my_string ="Rust Web";
    match my_string {
        "Rust Web"=>println!("Hello world, Rust Web!"),
        "Hi"=>println!("Hi, Rust Async!"),
         _ =>println!(" None !"),
    }
}

fn call_vector() {
    let mut vec1 = vec![11, 22, 33];
    let mut vec2 = vec![44, 55, 66];
    vec1.append(&mut vec2);
    println!("vec1: {:?}", vec1);
    println!("vec2: {:?}", vec2);
    // Output:
    //vec1: [11,22,33, 44,55,66]
    //vec2: []

    let mut vec1 = vec![11, 22, 33];
    let vec2 = vec![44,55,66];
    vec1.extend(&vec2);
    println!("vec1: {:?}", vec1);
    println!("vec2: {:?}", vec2);
    // Output:
    // vec1: [11,22,33, 44,55,66]
    // vec2: [44,55,66]

    let vec1 = vec![11,22,33];
    let vec2 = vec![44,55,66];
    let vec3 = vec![vec1, vec2].concat();
    println!("vec3: {:?}", vec3);
    // Output:
    // vec1: [11,22,33]
    // vec2: [44,55,66]
    // vec3: [11,22,33, 44,55,66]
    do_match();
}
fn main() {
    // call common fn
    call_common();
    call_vector();

}
