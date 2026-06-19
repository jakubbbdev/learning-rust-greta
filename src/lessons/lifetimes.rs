use super::section;

/// Demonstrates lifetime annotations on references.
pub fn run() {
    section("Lesson 13: Lifetimes");

    let text1 = String::from("Rust");
    let text2 = String::from("Rust language");

    // `'a` says: the returned reference lives as long as both inputs.
    let result = longest(text1.as_str(), text2.as_str());
    println!("  longest: \"{result}\"");

    // Static reference: valid for the entire program.
    let greeting = get_greeting();
    println!("  static str: {greeting}");

    // Structs that hold references also need lifetime annotations.
    let excerpt = Excerpt {
        text: "Hello Rust learners",
    };
    println!("  first word: \"{}\"", excerpt.first_word());

    println!();
    println!("  Notes:");
    println!("  - lifetimes describe how long references are valid");
    println!("  - most lifetimes are inferred by the compiler");
    println!("  - write them when the compiler cannot connect input and output refs");
}

/// Returns the longer of two string slices.
/// `'a` means the returned reference lives as long as both arguments.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/// Returns a string slice with `'static` lifetime.
fn get_greeting() -> &'static str {
    "hello"
}

/// Holds a borrowed string slice inside a struct.
struct Excerpt<'a> {
    text: &'a str,
}

impl<'a> Excerpt<'a> {
    /// Returns the first word of the stored text.
    fn first_word(&self) -> &str {
        self.text.split_whitespace().next().unwrap_or("")
    }
}
