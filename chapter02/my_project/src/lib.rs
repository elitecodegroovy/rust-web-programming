
pub mod utils {
    pub fn do_help() {
        println!("Helper function calling");
    }

    pub fn schedule_help() {
        do_help()
    }
}

pub mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

pub fn call_help() {
    crate::utils::do_help();
}

#[allow(dead_code)]
pub fn call_func() {
    utils::do_help();
}