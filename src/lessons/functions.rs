use super::section;

/// Demonstrates functions, return values, and closures.
pub fn run() {
    section("Lesson 3: Functions");

    let result = add(3, 4);
    println!("  add(3, 4) = {result}");

    greet("world");

    // Functions can return multiple values as a tuple.
    let (quotient, remainder) = div_with_remainder(17, 5);
    println!("  17 / 5: quotient {quotient}, remainder {remainder}");

    // Closure: a small anonymous function. Can read variables from outer scope.
    let factor = 3;
    let multiply = |x| x * factor;
    println!("  closure: 7 * 3 = {}", multiply(7));

    println!();
    println!("  Notes:");
    println!("  - last expression without semicolon is the return value");
    println!("  - parameter types are required");
    println!("  - closures can capture variables from the scope");
}

/// Adds two integers and returns the sum.
/// The last expression without `;` is returned automatically.
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Prints a greeting. Returns nothing (`()`).
/// `&str` means the function borrows a string slice; it does not take ownership.
fn greet(name: &str) {
    println!("  Hello, {name}!");
}

/// Returns quotient and remainder of integer division.
fn div_with_remainder(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}
