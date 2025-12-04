use std::io::stdin;


#[allow(dead_code)]
pub fn get_name() -> String {
    let mut name = String::new();
    println!("Please enter your name");
    stdin().read_line(&mut name).unwrap();
    name
}