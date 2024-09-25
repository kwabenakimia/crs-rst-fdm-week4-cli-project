/// Calculate the factorial of a number
/// # Examples
/// ```
/// use crs_rst_fdm_week4_cli_project::factorial::factorial;
/// let result = factorial(5);
/// let expected_output = 120u32;
/// assert_eq!(result, expected_output);
/// ```
pub fn factorial(n: u128) -> u128 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::factorial;

    #[test]
    fn test_factorial() {
        let result = factorial(5);
        let expected_output = 120;
        assert_eq!(result, expected_output);
    }
}
