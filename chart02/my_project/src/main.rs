
use my_project::utils::do_help;
use my_project::math::add;
use my_project::call_help;

fn main() {
    call_help();
    do_help();
    println!("2 + 3 = {}", add(10, 20));
}