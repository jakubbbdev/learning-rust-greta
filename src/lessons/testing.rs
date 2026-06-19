use super::section;

/// Demonstrates unit tests and the basic test layout in Rust.
pub fn run() {
    section("Lesson 14: Testing");

    println!("  add(2, 3) = {}", add(2, 3));
    println!("  is_even(4) = {}", is_even(4));
    println!("  is_even(5) = {}", is_even(5));

    // Tests live in a `tests` module, often at the bottom of the same file.
    // Run them with: cargo test
    println!();
    println!("  Run tests with: cargo test");
    println!("  This lesson includes example tests in the same file.");

    println!();
    println!("  Notes:");
    println!("  - tests go in #[cfg(test)] modules");
    println!("  - assert!, assert_eq!, and assert_ne! check results");
    println!("  - cargo test runs all tests in the project");
}

/// Adds two numbers. Kept simple so tests can verify it.
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Returns true when the number is divisible by 2.
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::{add, is_even};

    #[test]
    fn add_works() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn is_even_works() {
        assert!(is_even(4));
        assert!(!is_even(5));
    }
}
