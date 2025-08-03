
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// Add together two i32 numbers and return the result of that addition
/// ```
/// assert_eq!(testing::add_i(2, 2), 4);
/// ```
///
/// ```
/// use testing::add_i;
/// assert_eq!(add_i(9, 2), 11);
/// ```
///
/// ```
/// use testing::add_i;
/// assert_eq!(add_i(70, 20), 90);
/// ```
///
pub fn add_i(x: i32, y: i32) -> i32 {
    x + y
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works2() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_add_i() {
        assert_eq!(add_i(4, 122), 126);
    }

}
