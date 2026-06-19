use super::section;

/// Demonstrates ownership, borrowing, mutable references, and slices.
pub fn run() {
    section("Lesson 5: Ownership and Borrowing");

    // Each value has one owner. When ownership moves, the old variable is invalid.
    let s1 = String::from("hello");
    let s2 = s1;
    println!("  moved s1 to s2: \"{s2}\"");

    // `clone` creates a deep copy when you need two owners.
    let s3 = s2.clone();
    println!("  clone: s2=\"{s2}\", s3=\"{s3}\"");

    // `&T` borrows without taking ownership. The owner stays valid.
    let text = String::from("Rust");
    let len = calculate_length(&text);
    println!("  length of \"{text}\": {len}");

    // `&mut T` borrows mutably. Only one mutable borrow at a time is allowed.
    let mut message = String::from("Hi");
    append_rust(&mut message);
    println!("  after mutable borrow: \"{message}\"");

    println!();
    println!("  Borrowing rules:");
    println!("  - one mutable reference OR many immutable references");
    println!("  - references must always be valid");
    println!("  - owner drop frees the memory");

    // A slice is a borrowed part of a string or array.
    let sentence = String::from("hello Rust world");
    let first = first_word(&sentence);
    println!("  first word: \"{first}\"");
}

/// Borrows a String immutably and returns its length.
/// Does not take ownership, so `text` remains usable after the call.
fn calculate_length(s: &String) -> usize {
    s.len()
}

/// Borrows a String mutably and appends text in place.
fn append_rust(s: &mut String) {
    s.push_str(" Rust!");
}

/// Returns the first word as a string slice.
/// Accepts `&str`, so it works with both String and string literals.
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}
