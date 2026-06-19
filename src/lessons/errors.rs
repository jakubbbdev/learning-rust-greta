use super::section;

/// Demonstrates Option, Result, and basic error handling.
pub fn run() {
    section("Lesson 8: Option, Result, and Errors");

    // Option<T>: Some(value) means present, None means missing.
    let found = find_number(3);
    let missing = find_number(99);

    match found {
        Some(n) => println!("  found: {n}"),
        None => println!("  not found"),
    }

    // unwrap_or gives a default when the value is None.
    let value = missing.unwrap_or(0);
    println!("  unwrap_or on None: {value}");

    // Result<T, E>: Ok(value) for success, Err(error) for failure.
    match divide(10.0, 2.0) {
        Ok(result) => println!("  10 / 2 = {result}"),
        Err(e) => println!("  error: {e}"),
    }

    match divide(10.0, 0.0) {
        Ok(result) => println!("  result: {result}"),
        Err(e) => println!("  10 / 0 error: {e}"),
    }

    match parse_age("25") {
        Ok(age) => println!("  parsed age: {age}"),
        Err(e) => println!("  parse error: {e}"),
    }

    println!();
    println!("  Notes:");
    println!("  - use Option instead of null");
    println!("  - use Result for recoverable errors");
    println!("  - avoid unwrap in production code");
    println!("  - ? propagates errors in Result-returning functions");
}

/// Searches a fixed list and returns Some if the number exists.
fn find_number(target: i32) -> Option<i32> {
    let numbers = [1, 2, 3, 4, 5];
    for &n in &numbers {
        if n == target {
            return Some(n);
        }
    }
    None
}

/// Divides two numbers. Returns Err when dividing by zero.
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("division by zero"))
    } else {
        Ok(a / b)
    }
}

/// Parses a string into u32. Returns Err if the text is not a valid number.
/// The `?` operator could replace this manual match in larger functions.
fn parse_age(input: &str) -> Result<u32, std::num::ParseIntError> {
    input.parse::<u32>()
}
