// raii.rs
#[allow(dead_code)]
pub fn create_box() {
    // Allocate an integer on the heap
    let _box1 = Box::new(3i32);

    // `_box1` is destroyed here, and memory gets freed
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

    }
}
