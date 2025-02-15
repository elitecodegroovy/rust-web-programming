#[allow(dead_code)]
pub mod common;
pub mod transfer;
pub mod user;
pub mod cli;
pub mod db;
pub mod controllers;
pub mod models;
pub mod services;
#[cfg(test)]
mod tests {}


fn now_millis() -> u64 {
    use std::time::SystemTime;
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

#[allow(dead_code)]
fn now_millis_i64() -> i64 {
    use std::time::SystemTime;
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

#[allow(dead_code)]
fn now_second_i32() -> i32 {
    use std::time::SystemTime;
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i32
}