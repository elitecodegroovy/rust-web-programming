#[cfg(target_os = "linux")]
const OS_MESSAGE: &str = "You are running linux!";

#[cfg(not(target_os = "linux"))]
const OS_MESSAGE: &str = "You are *not* running linux!";

fn main() {
    println!("{}", OS_MESSAGE);

    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}