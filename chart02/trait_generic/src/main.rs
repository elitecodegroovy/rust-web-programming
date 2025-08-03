trait Processor {
    fn process<T>(&self, data: T) -> T;
}

struct IdentityProcessor;

impl Processor for IdentityProcessor {
    fn process<T>(&self, data: T) -> T {
        data
    }
}

fn main() {
    let processor = IdentityProcessor;
    let result = processor.process(99);  // Returns 99
    println!("{}", result);
    let result_str = processor.process("Rust");  // Returns "Rust"
    println!("{}", result_str);
}
