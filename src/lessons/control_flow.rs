use super::section;

/// Demonstrates if expressions, loops, while, and for.
pub fn run() {
    section("Lesson 4: Control Flow");

    // In Rust, `if` is an expression and returns a value.
    // All branches must return the same type.
    let score = 85;
    let label = if score >= 90 {
        "Excellent"
    } else if score >= 75 {
        "Good"
    } else {
        "Keep practicing"
    };
    println!("  score {score}: {label}");

    // `for` is the usual way to iterate over ranges or collections.
    // 1..=3 includes 3. 1..3 would stop at 2.
    print!("  for 1..=3: ");
    for i in 1..=3 {
        print!("{i} ");
    }
    println!();

    // `while` repeats as long as the condition is true.
    let mut n = 3;
    print!("  while n > 0: ");
    while n > 0 {
        print!("{n} ");
        n -= 1;
    }
    println!();

    // `loop` runs forever until `break`. `break` can return a value.
    let mut counter = 0;
    let total = loop {
        counter += 1;
        if counter == 5 {
            break counter * 10;
        }
    };
    println!("  loop break value: {total}");

    println!();
    println!("  Notes:");
    println!("  - if is an expression");
    println!("  - 1..3 is exclusive, 1..=3 is inclusive");
    println!("  - for is common for ranges and collections");
}
