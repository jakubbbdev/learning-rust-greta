use super::section;

/// Demonstrates enums, match, if let, and while let.
pub fn run() {
    section("Lesson 7: Enums and Pattern Matching");

    // Enum variants can be simple tags or carry extra data.
    let active = Status::Active;
    let inactive = Status::Inactive;
    let text_msg = Message::Text(String::from("Hello"));
    let number_msg = Message::Number(42);
    let empty_msg = Message::Empty;

    describe_status(&active);
    describe_status(&inactive);
    describe_message(&text_msg);
    describe_message(&number_msg);
    describe_message(&empty_msg);

    let roll = Dice::Three;
    println!("  dice roll: {} points", score(roll));

    print!("  all dice values: ");
    for die in [
        Dice::One,
        Dice::Two,
        Dice::Three,
        Dice::Four,
        Dice::Five,
        Dice::Six,
    ] {
        print!("{} ", score(die));
    }
    println!();

    // `if let` handles one pattern and skips the rest.
    let config = Some("dark");
    if let Some(theme) = config {
        println!("  theme: {theme}");
    }

    // `while let` repeats while the pattern matches.
    let mut stack = vec![1, 2, 3];
    print!("  stack pop: ");
    while let Some(n) = stack.pop() {
        print!("{n} ");
    }
    println!();

    println!();
    println!("  Notes:");
    println!("  - enums can hold data per variant");
    println!("  - match must cover all variants");
    println!("  - if let and while let match one pattern");
}

/// Simple on/off status without extra data.
enum Status {
    Active,
    Inactive,
}

/// Message variants with different attached data types.
enum Message {
    Text(String),
    Number(i32),
    Empty,
}

/// Dice sides used to show exhaustive matching.
#[derive(Copy, Clone)]
enum Dice {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

/// Maps each dice variant to a numeric score.
/// `match` must handle every variant or the code will not compile.
fn score(die: Dice) -> u8 {
    match die {
        Dice::One => 1,
        Dice::Two => 2,
        Dice::Three => 3,
        Dice::Four => 4,
        Dice::Five => 5,
        Dice::Six => 6,
    }
}

/// Prints a human-readable status using pattern matching.
fn describe_status(status: &Status) {
    match status {
        Status::Active => println!("  status: active"),
        Status::Inactive => println!("  status: inactive"),
    }
}

/// Extracts data from a Message variant and prints it.
fn describe_message(msg: &Message) {
    match msg {
        Message::Text(text) => println!("  message text: \"{text}\""),
        Message::Number(n) => println!("  message number: {n}"),
        Message::Empty => println!("  message: empty"),
    }
}
