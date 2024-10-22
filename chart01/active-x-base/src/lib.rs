use std::fmt::{Debug, Display};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// This function only gets compiled if the target OS is linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

// And this function only gets compiled if the target OS is *not* linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!");
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: `{:?}`", t);
    println!("u: `{:?}`", u);
}

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: `{:?}`", t);
    println!("Display: `{}`", t);
}

trait PrintInOption {
    fn print_in_option(self);
}

// Because we would otherwise have to express this as `T: Debug` or
// use another method of indirect approach, this requires a `where` clause:
impl<T> PrintInOption for T where
    Option<T>: Debug {
    // We want `Option<T>: Debug` as our bound because that is what's
    // being printed. Doing otherwise would be using the wrong bound.
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

struct Years(i64);

struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}


impl Days {
    /// truncates partial years
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn is_adult(age: &Years) -> bool {
    age.0 >= 18
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        are_you_on_linux();
        let string = "Rust";

        let result = add(2, 2);
        assert_eq!(result, 4);

        let array = [1, 2, 3];
        let vec = vec![1, 2, 3];

         compare_prints(&string);

        compare_types(&array, &vec);

        let vec = vec![1, 2, 3];
        vec.print_in_option();

        let age = Years(25);
        let age_days = age.to_days();
        println!("Is an adult? {}", is_adult(&age));
        println!("Is an adult? {}", is_adult(&age_days.to_years()));
        println!("Is an adult? {}", is_adult(&age));
    }
}
