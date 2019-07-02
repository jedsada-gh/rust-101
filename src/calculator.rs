use std::io;
use std::io::prelude::*;

fn main() {
    println!("Welcome Calculator Application");
    println!("1. Plus");
    println!("2. Minus");
    println!("3. Multiply");
    println!("4. Divide");
    println!("Select Operator pls: ");
    let operator = get_input();
    println!("Enter number 1: ");
    let number_1 = get_input();
    println!("Enter number 2: ");
    let number_2 = get_input();
    let result = calculate(operator, number_1, number_2);
    if result.1.chars().count() > 0 {
        print!("Error: {}", result.1);
    } else {
        print!("Result: {}", result.0);
    }
}

fn get_input() -> i32 {
    let stdin = io::stdin();
    let mut input_number: i32 = 0;
    for line in stdin.lock().lines() {
        input_number = line.unwrap().trim().parse().expect("please give me correct string number!");
        break;
    }
    return input_number;
}

fn calculate(operator: i32, number_1: i32, number_2: i32) -> (i32, String) {
    let empty_message = String::from("");
    let dvide_by_zero_message = String::from("Divide by zero");
    let result = if operator == 1 {
        (number_1 + number_2, empty_message)
    } else if operator == 2 {
        (number_1 - number_2, empty_message)
    } else if operator == 3 {
        (number_1 * number_2, empty_message)
    } else {
        if number_2 == 0 {
            (0, dvide_by_zero_message)
        } else {
            (number_1 / number_2, empty_message)
        }
    };
    return result;
}