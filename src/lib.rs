//! Rust fundamentals - a teaching library with runnable lesson modules.
//!
//! Run all lessons from the binary: `cargo run`
//! Run a single lesson from code: `learning_rust_greta::lessons::ownership::run()`

pub mod lessons;

pub use lessons::run_all;
