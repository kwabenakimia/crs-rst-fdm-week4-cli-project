use std::io;

fn main() {
    loop {
        println!("Choose an operation: add, subtract, multiply, divide, factorial");
        let mut operation = String::new();
        io::stdin().read_line(&mut operation).expect("Failed to read line");
        let operation = operation.trim();

        match operation {
            "add" | "subtract" | "multiply" | "divide" => {
                let (num1, num2) = get_two_numbers();
                let result = match operation {
                    "add" => num1 + num2,
                    "subtract" => num1 - num2,
                    "multiply" => num1 * num2,
                    "divide" => num1 / num2,
                    _ => unreachable!(),
                };
                println!("The result is: {}", result);
            }
            "factorial" => {
                let num = get_one_number();
                let result = factorial(num);
                println!("The result is: {}", result);
            }
            _ => {
                println!("Invalid operation");
                continue;
            }
        }

        println!("Do you want to perform another operation? (yes/no)");
        let mut again = String::new();
        io::stdin().read_line(&mut again).expect("Failed to read line");
        if again.trim().to_lowercase() != "yes" {
            break;
        }
    }
}

fn get_two_numbers() -> (i32, i32) {
    println!("Enter the first number:");
    let num1 = read_number();
    println!("Enter the second number:");
    let num2 = read_number();
    (num1, num2)
}

fn get_one_number() -> i32 {
    println!("Enter the number:");
    read_number()
}

fn read_number() -> i32 {
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Failed to read line");
    num.trim().parse().expect("Please enter a valid number")
}

fn factorial(n: i32) -> i32 {
    (1..=n).product()
}