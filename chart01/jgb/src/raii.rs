use std::{ thread};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_raii() {
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
    }
}
