use std::thread;

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
    }
}
