use crs_rst_fdm_week4_cli_project::factorial::factorial;
use crs_rst_fdm_week4_cli_project::basic_math::{add, subtract, multiply, divide};
use std::io::{self, Write};

fn main() {
    loop {
        if !run_calculator(&mut io::stdin().lock(), &mut io::stdout()) {
            break;
        }
    }
}

fn run_calculator<R: io::BufRead, W: Write>(input: &mut R, output: &mut W) -> bool {
    writeln!(output, "Choose an operation with its letter: add(a), subtract(s), multiply(m), divide(d), factorial(f)").unwrap();
    let mut operation = String::new();
    input.read_line(&mut operation).expect("Failed to read line");
    let operation = operation.trim();

    match operation {
        "a" | "s" | "m" | "d" => {
            let (num1, num2) = get_two_numbers(input, output);
            let result = match operation {
                "a" => add(num1, num2),
                "s" => subtract(num1, num2),
                "m" => multiply(num1, num2),
                "d" => divide(num1, num2),
                _ => unreachable!(),
            };
            writeln!(output, "The result is: {}", result).unwrap();
        }
        "f" => {
            let num = get_one_number(input, output);
            let result = factorial(num);
            writeln!(output, "The result is: {}", result).unwrap();
        }
        _ => {
            writeln!(output, "Invalid operation").unwrap();
            return true;
        }
    }

    writeln!(output, "Do you want to perform another operation? (yes/no)").unwrap();
    let mut again = String::new();
    input.read_line(&mut again).expect("Failed to read line");
    again.trim().to_lowercase() == "yes"
}

fn get_two_numbers<R: io::BufRead, W: Write>(input: &mut R, output: &mut W) -> (f32, f32) {
    writeln!(output, "Enter the first number:").unwrap();
    let num1 = read_number(input) as f32;
    writeln!(output, "Enter the second number:").unwrap();
    let num2 = read_number(input) as f32;
    (num1, num2)
}

fn get_one_number<R: io::BufRead, W: Write>(input: &mut R, output: &mut W) -> u128 {
    writeln!(output, "Enter the number:").unwrap();
    read_number(input) as u128
}

fn read_number<R: io::BufRead>(input: &mut R) -> i32 {
    let mut num = String::new();
    input.read_line(&mut num).expect("Failed to read line");
    num.trim().parse().expect("Please enter a valid number")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_addition() {
        let input = b"a\n2\n3\nno\n";
        let mut output = Vec::new();
        let mut cursor = Cursor::new(input as &[u8]);

        run_calculator(&mut cursor, &mut output);

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("The result is: 5"));
    }

    #[test]
    fn test_factorial() {
        let input = b"f\n5\nno\n";
        let mut output = Vec::new();
        let mut cursor = Cursor::new(input as &[u8]);

        run_calculator(&mut cursor, &mut output);

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("The result is: 120"));
    }
}