use super::section;

/// Demonstrates structs, impl blocks, methods, and tuple structs.
pub fn run() {
    section("Lesson 6: Structs and Methods");

    // Struct fields are named and typed. Each field owns or borrows its data.
    let user = User {
        name: String::from("Greta"),
        age: 25,
        active: true,
    };
    println!("  user: {} ({}), active={}", user.name, user.age, user.active);

    // `..other` fills missing fields from another struct instance.
    let user2 = User {
        name: String::from("Max"),
        ..user
    };
    println!("  user2: {} ({}), active={}", user2.name, user2.age, user2.active);

    // Associated function: called as Account::new(), no instance needed yet.
    let account = Account::new("Anna", 100.0);
    println!("  account: {} has {:.2}", account.owner, account.balance);

    // Methods take `&self` or `&mut self` as the first parameter implicitly.
    let mut account = account;
    account.deposit(50.0);
    account.withdraw(30.0);
    println!("  balance after changes: {:.2}", account.balance);

    // Tuple struct: fields are accessed by index instead of names.
    let color = Color(255, 128, 0);
    println!("  tuple struct: ({}, {}, {})", color.0, color.1, color.2);

    println!();
    println!("  Notes:");
    println!("  - structs group related data");
    println!("  - impl defines methods");
    println!("  - associated functions have no self, e.g. Account::new()");
}

/// A simple user record with owned string data.
struct User {
    name: String,
    age: u32,
    active: bool,
}

/// A bank account with an owner name and balance.
struct Account {
    owner: String,
    balance: f64,
}

impl Account {
    /// Creates a new account with a starting balance.
    /// Called as `Account::new(...)`, not on an existing instance.
    fn new(owner: &str, starting_balance: f64) -> Self {
        Self {
            owner: String::from(owner),
            balance: starting_balance,
        }
    }

    /// Adds money to the balance. Needs `&mut self` to modify the struct.
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    /// Removes money if enough balance is available.
    fn withdraw(&mut self, amount: f64) {
        if amount <= self.balance {
            self.balance -= amount;
        }
    }
}

/// RGB color stored as three byte values.
struct Color(u8, u8, u8);
