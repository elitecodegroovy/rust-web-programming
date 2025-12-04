

fn main() {
    let a = 0;
    let mut b = 1;
    println!(" >>> a={}, b={}", a, b);

    b += 10;
    println!(" >>> a={}, b={}", a, b);

    let mut s = String::with_capacity(10_000_000);
    for i in 0..10_000_000 {
        s.push('.')
    }
    println!("x len {}", s.len());

}
