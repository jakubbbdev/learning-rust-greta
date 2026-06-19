use super::section;

/// Demonstrates variables, mutability, shadowing, and constants.
pub fn run() {
    section("Lesson 1: Variables and Mutability");

    // `let` creates an immutable binding. The value cannot change later.
    let x = 5;
    println!("  let x = 5;           x = {x}");

    // `mut` allows the binding to be changed. Use it only when needed.
    let mut counter = 0;
    counter += 1;
    println!("  mut counter += 1;    counter = {counter}");

    // Shadowing: declare a new binding with the same name.
    // The old binding is hidden. The type can change.
    let text = "hello";
    let text = text.len();
    println!("  shadowing: text = {text} (type changed)");

    // `const` is always immutable and must have a known type.
    // Valid for the whole program, not just one scope.
    const MAX_POINTS: u32 = 100;
    println!("  const MAX_POINTS = {MAX_POINTS}");

    println!();
    println!("  Notes:");
    println!("  - bindings are immutable by default");
    println!("  - use mut only when needed");
    println!("  - shadowing is not the same as mut");
}
