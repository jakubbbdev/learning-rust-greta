use super::section;

/// Demonstrates generic functions, structs, enums, and methods.
pub fn run() {
    section("Lesson 10: Generics");

    // Generic function: one implementation, many types.
    println!("  largest of 3 numbers: {}", largest(&[3, 7, 2]));
    println!("  largest of 3 chars: {}", largest(&['a', 'z', 'm']));

    // Generic struct: fields can have different type parameters.
    let pair_int = Pair { first: 1, second: 2 };
    let pair_text = Pair {
        first: "hello",
        second: "world",
    };
    println!("  pair int: ({}, {})", pair_int.first, pair_int.second);
    println!("  pair text: ({}, {})", pair_text.first, pair_text.second);

    // Generic enum: Option and Result from earlier lessons use this idea.
    let maybe_number: Option<i32> = Some(10);
    let maybe_text: Option<&str> = None;
    println!("  Option<i32>: {:?}", maybe_number);
    println!("  Option<&str>: {:?}", maybe_text);

    // Methods can be generic too, often with trait bounds.
    let numbers = vec![1, 2, 3];
    let texts = vec!["a", "bb", "ccc"];
    println!("  show items: {}", show_items(&numbers));
    println!("  show items: {}", show_items(&texts));

    println!();
    println!("  Notes:");
    println!("  - generics avoid duplicate code for different types");
    println!("  - type parameters are written as <T> or <T, U>");
    println!("  - trait bounds add requirements, e.g. T: Display");
}

/// Returns the largest item in a slice.
/// `T: Ord` means T must support comparison with `>`.
fn largest<T: Ord + Copy>(items: &[T]) -> T {
    let mut max = items[0];
    for &item in items {
        if item > max {
            max = item;
        }
    }
    max
}

/// Holds two values that may have different types.
struct Pair<T, U> {
    first: T,
    second: U,
}

/// Converts a slice into a comma-separated string.
/// `T: Display` means items must be printable with `{}`.
fn show_items<T: std::fmt::Display>(items: &[T]) -> String {
    items
        .iter()
        .map(|item| item.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}
