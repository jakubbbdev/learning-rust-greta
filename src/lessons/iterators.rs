use super::section;

/// Demonstrates iterators, adapter methods, and collecting results.
pub fn run() {
    section("Lesson 12: Iterators");

    let numbers = vec![1, 2, 3, 4, 5];

    // iter() borrows items. into_iter() takes ownership. iter_mut() borrows mutably.
    print!("  original: ");
    for n in &numbers {
        print!("{n} ");
    }
    println!();

    // map transforms each item and returns a new iterator (lazy).
    let doubled: Vec<i32> = numbers.iter().map(|n| n * 2).collect();
    println!("  doubled: {:?}", doubled);

    // filter keeps items that match a condition.
    let evens: Vec<i32> = numbers.iter().copied().filter(|n| n % 2 == 0).collect();
    println!("  evens: {:?}", evens);

    // fold reduces an iterator to one value.
    let sum = numbers.iter().sum::<i32>();
    let product = numbers.iter().fold(1, |acc, n| acc * n);
    println!("  sum: {sum}, product: {product}");

    // Chaining adapter methods is common in Rust.
    let result: Vec<i32> = numbers
        .iter()
        .filter(|n| **n > 2)
        .map(|n| n + 10)
        .collect();
    println!("  filtered and mapped: {:?}", result);

    // enumerate gives index and value.
    print!("  enumerate: ");
    for (index, value) in numbers.iter().enumerate() {
        print!("({index}:{value}) ");
    }
    println!();

    println!();
    println!("  Notes:");
    println!("  - iterators are lazy until collect or a consuming method runs");
    println!("  - map, filter, and fold are very common");
    println!("  - prefer iterators over manual index loops when possible");
}
