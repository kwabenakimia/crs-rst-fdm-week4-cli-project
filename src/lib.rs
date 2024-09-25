//! This is a library that provides utilities for command-line tools.
//! Create a library that provides basic mathematical utility functions.
//! Include functions like calculating the factorial of a number,
//! finding the greatest common divisor (GCD) of two numbers, and checking if a number is prime.
//!
//! # Examples:
//! ```
//! use crs_rst_fdm_week4_cli_project::basic_math::add;
//! let result = add(7.0,7.0);
//! let expected_output = 14.0;
//! assert_eq!(result, expected_output);
//!
//! use crs_rst_fdm_week4_cli_project::factorial::factorial;
//! let result = factorial(5);
//! let expected_output = 120u32;
//! assert_eq!(result, expected_output);
//! ```
//! # Panics:
//! The `read_stdin` function will panic if it fails to read a line with a message "Failed to read input line".

pub mod basic_math;
pub mod factorial;

#[cfg(test)]
mod tests {
    use super::basic_math::add;

    #[test]
    fn test_add_input() {
        let result = add(2.0, 3.0);
        let expected_output = 5.0;
        assert_eq!(result, expected_output);
    }
}
