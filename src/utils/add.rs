/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// use playground::utils::add::*;
///
/// let result = do_add(5, 10);
/// assert_eq!(result, 15);
/// ```
///
pub fn do_add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do_add() {
        assert_eq!(do_add(5, 10), 15);
    }
}