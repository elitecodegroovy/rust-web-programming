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

use std::iter;
use std::vec::IntoIter;

#[allow(dead_code)]
// This function combines two `Vec<i32>` and returns an iterator over it.
// Look how complicated its return type is!
fn combine_vecs_explicit_return_type(
    v: Vec<i32>,
    u: Vec<i32>,
) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

// This is the exact same function, but its return type uses `impl Trait`.
// Look how much simpler it is!
fn combine_vecs(
    v: Vec<i32>,
    u: Vec<i32>,
) -> impl Iterator<Item=i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

// Returns a function that adds `y` to its input
fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    let closure = move |x: i32| { x + y };
    closure
}

fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
    numbers.iter()
        .filter(|x| x > &&0)
        .map(|x| x * 142857)
}

trait UsernameWidget {
    // Get the selected username out of this widget
    fn get(&self) -> String;
}

trait AgeWidget {
    // Get the selected age out of this widget
    fn get(&self) -> u8;
}

// A form with both a UsernameWidget and an AgeWidget
struct Form {
    username: String,
    age: u8,
}

impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}

impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
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

        // combine the vectors
        let v1 = vec![1, 2, 3];
        let v2 = vec![4, 5];
        let mut v3 = combine_vecs(v1, v2);
        assert_eq!(Some(1), v3.next());
        assert_eq!(Some(2), v3.next());
        assert_eq!(Some(3), v3.next());
        assert_eq!(Some(4), v3.next());
        assert_eq!(Some(5), v3.next());

        let plus_one = make_adder_function(1);
        assert_eq!(plus_one(2), 3);

        let singles = vec![-3, -2, 2, 3, 4, 5,6,7];
        let doubles = double_positives(&singles);
        println!(">>> positive double {:?}", doubles.collect::<Vec<i32>>());

        let form = Form {
            username: "rustacean".to_owned(),
            age: 28,
        };
        // If you uncomment this line, you'll get an error saying
        // "multiple `get` found". Because, after all, there are multiple methods
        // named `get`.
        // println!("{}", form.get());
        let username = <Form as UsernameWidget>::get(&form);
        assert_eq!("rustacean".to_owned(), username);
        let age = <Form as AgeWidget>::get(&form);
        assert_eq!(28, age);
    }
}
