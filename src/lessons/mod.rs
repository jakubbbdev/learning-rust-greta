//! Core Rust topics, one module per lesson.
//!
//! Each file contains:
//! - a short `run()` demo for console output
//! - comments on important lines
//! - doc comments on functions, structs, and enums

pub mod variables;
pub mod types;
pub mod functions;
pub mod control_flow;
pub mod ownership;
pub mod structs;
pub mod enums;
pub mod errors;
pub mod traits;
pub mod generics;
pub mod collections;
pub mod iterators;
pub mod lifetimes;
pub mod testing;

use std::io::{self, Write};

/// Runs every lesson in order.
pub fn run_all() {
    variables::run();
    types::run();
    functions::run();
    control_flow::run();
    ownership::run();
    structs::run();
    enums::run();
    errors::run();
    traits::run();
    generics::run();
    collections::run();
    iterators::run();
    lifetimes::run();
    testing::run();
}

/// Prints a simple heading between lessons.
pub fn section(title: &str) {
    println!();
    println!("--- {title} ---");
    println!();
}

/// Optional pause between lessons.
pub fn pause() {
    print!("Press Enter for the next lesson...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}
