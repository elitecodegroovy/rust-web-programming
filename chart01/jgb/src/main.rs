mod raii;
use std::env;

#[warn(unused_variables)]
struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("Destructor ToDrop have been executed.");
    }
}

fn main() {
    println!("Jin gu bang!");
    // Creating lots of boxes just for fun
    // There's no need to manually free memory!
    for _ in 0u32..1_000 {
        raii::create_box();
    }

    let _x = ToDrop;
    println!("Made a ToDrop!");

    let args: Vec<String> = env::args().collect();

    // The first argument is the path that was used to call the program.
    println!("My path is {}.", args[0]);

    // The rest of the arguments are the passed command line parameters.
    // Call the program like this:
    //   $ ./args arg1 arg2
    println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
}

//  sudo yum install -y glibc glibc-debuginfo # OpenEuler System
// valgrind ./raii
// Valgrind is a GPL'd system for debugging and profiling Linux programs. With Valgrind's tool suite you can automatically detect many memory management and threading bugs, avoiding hours of frustrating bug-hunting, making your programs more stable. You can also perform detailed profiling to help speed up your programs.
