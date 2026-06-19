use super::section;
use std::collections::HashMap;

/// Demonstrates Vec and HashMap, common growable collections.
pub fn run() {
    section("Lesson 11: Collections");

    // Vec: growable array on the heap.
    let mut scores = Vec::new();
    scores.push(90);
    scores.push(85);
    scores.push(72);
    println!("  vec: {:?}", scores);
    println!("  first score: {}", scores[0]);

    // vec! macro creates a vector with initial values.
    let names = vec!["Anna", "Ben", "Clara"];
    print!("  names: ");
    for name in &names {
        print!("{name} ");
    }
    println!();

    // HashMap: key-value pairs. Keys must be hashable.
    let mut ages = HashMap::new();
    ages.insert("Anna", 28);
    ages.insert("Ben", 31);
    ages.insert("Clara", 25);
    println!("  Anna age: {}", ages["Anna"]);

    // Inserting updates an existing key.
    ages.insert("Anna", 29);
    println!("  Anna updated age: {}", ages.get("Anna").unwrap());

    // Safe lookup with get returns Option.
    match ages.get("Unknown") {
        Some(age) => println!("  found age: {age}"),
        None => println!("  key not found"),
    }

    // Iterate over key-value pairs.
    print!("  ages: ");
    for (name, age) in &ages {
        print!("{name}={age} ");
    }
    println!();

    println!();
    println!("  Notes:");
    println!("  - Vec stores items in order and can grow");
    println!("  - HashMap stores unique keys with values");
    println!("  - use get for safe lookup instead of []");
}
