use super::section;

/// Demonstrates scalar types, strings, tuples, arrays, and slices.
pub fn run() {
    section("Lesson 2: Data Types");

    // Integer types differ in size and signedness (u = unsigned, i = signed).
    let small: u8 = 255;
    let large: i64 = -1_000_000;
    println!("  u8: {small}, i64: {large}");

    // f32 and f64 store floating-point numbers.
    let pi: f64 = 3.14159;
    println!("  f64: {pi:.2}");

    // bool is true/false. char is a single Unicode character (4 bytes).
    let active: bool = true;
    let letter: char = 'R';
    println!("  bool: {active}, char: {letter}");

    // &str points to string data elsewhere. String owns heap-allocated text.
    let literal: &str = "Hello";
    let mut owned = String::from("Rust");
    owned.push('!');
    println!("  &str: \"{literal}\"");
    println!("  String: \"{owned}\"");

    // Tuple: fixed number of values, possibly different types. Access with .0, .1, ...
    let person: (&str, u32) = ("Anna", 28);
    println!("  tuple: name={}, age={}", person.0, person.1);

    // Array: fixed length, same type. Length is part of the type.
    let numbers: [i32; 3] = [10, 20, 30];
    println!("  array: {:?}", numbers);

    // Slice: borrowed view into a sequence. Written as &[T] or &str.
    let slice = &numbers[1..];
    println!("  slice: {:?}", slice);

    println!();
    println!("  Notes:");
    println!("  - types are usually inferred");
    println!("  - &str is borrowed, String is owned");
    println!("  - array length is fixed at compile time");
}
