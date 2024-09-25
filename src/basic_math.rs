/// This function reads adds two floats and returns the result.
/// # Examples:
/// ```
/// use crs_rst_fdm_week4_cli_project::basic_math::add;
/// let result = add(2.0, 3.0);
/// assert_eq!(result, 5.0);
/// ```
pub fn add(a: f32, b: f32) -> f32 {
    a + b
}

/// This function reads subtracts two floats and returns the result.
/// # Examples:
/// ```
/// use crs_rst_fdm_week4_cli_project::basic_math::subtract;
/// let result = subtract(5.0, 3.0);
/// assert_eq!(result, 2.0);
/// ```
pub fn subtract(a: f32, b: f32) -> f32 {
    a - b
}

/// This function reads multiplies two floats and returns the result.
/// # Examples:
/// ```
/// use crs_rst_fdm_week4_cli_project::basic_math::multiply;
/// let result = multiply(2.0, 3.0);
/// assert_eq!(result, 6.0);
/// ```
pub fn multiply(a: f32, b: f32) -> f32 {
    a * b
}

/// This function reads divides two floats and returns the result.
/// # Examples:
/// ```
/// use crs_rst_fdm_week4_cli_project::basic_math::divide;
/// let result = divide(6.0, 3.0);
/// assert_eq!(result, 2.0);
/// ```
pub fn divide(a: f32, b: f32) -> f32 {
    a / b
}

// create a function that loops and ask which operation to perform
// between add, subtract, multiply, divide or factorial
// then ask for the numbers to perform the operation on
// then print the result
// then ask if the user wants to perform another operation
// if yes, loop again
// if no, exit the program

#[cfg(test)]
mod tests {
    use super::{add, subtract, multiply, divide};

    #[test]
    fn test_add() {
        let result = add(2.0, 3.0);
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_subtract() {
        let result = subtract(5.0, 3.0);
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_multiply() {
        let result = multiply(2.0, 3.0);
        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_divide() {
        let result = divide(6.0, 3.0);
        assert_eq!(result, 2.0);
    }
}
