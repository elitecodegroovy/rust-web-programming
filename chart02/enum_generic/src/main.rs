use std::fmt::{Display , Formatter, Result as FmtResult};

enum MyResult<T> {
    Ok(T),
    Err(String)
}

impl  <T:Display> Display for MyResult<T> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            MyResult::Ok(v) => write!(f, "OK({})", v),
            MyResult::Err(msg) => write!(f, "Err({})", msg),
        }
    }
}

fn main() {
    let ok_result = MyResult::Ok(42);
    println!("{}", ok_result);  // Prints: Ok(42)

    // let err_result = MyResult::Err("something went wrong".to_string());
    // println!("{}", err_result);  // Prints: Err(something went wrong)
}
