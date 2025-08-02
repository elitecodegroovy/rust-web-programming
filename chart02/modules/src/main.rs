mod input;
mod output;

// use input::get_name;
use output::{goodbye, hello};

fn main() {
    let name = "John Lau";
    hello(&name);
    goodbye(&name);
}
