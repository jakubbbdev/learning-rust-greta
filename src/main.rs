//! Rust fundamentals - learning program.
//!
//! Run everything:  `cargo run`
//! Single lesson:   uncomment one line in `main()` below

use learning_rust_greta::{lessons, run_all};

fn main() {
    println!();
    println!("Rust Fundamentals");
    println!("14 lessons with short examples.");
    println!();

    // Run all lessons
    run_all();

    // Or run a single lesson (comment out run_all() above):
    // lessons::variables::run();
    // lessons::types::run();
    // lessons::functions::run();
    // lessons::control_flow::run();
    // lessons::ownership::run();
    // lessons::structs::run();
    // lessons::enums::run();
    // lessons::errors::run();
    // lessons::traits::run();
    // lessons::generics::run();
    // lessons::collections::run();
    // lessons::iterators::run();
    // lessons::lifetimes::run();
    // lessons::testing::run();

    lessons::section("Done");
    println!("Next steps:");
    println!("  - https://doc.rust-lang.org/book/");
    println!("  - https://rustlings.cool/");
    println!("  - Build a small CLI or library project");
    println!();
}
