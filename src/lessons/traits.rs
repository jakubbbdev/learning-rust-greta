use super::section;

/// Demonstrates traits, default methods, and trait bounds.
pub fn run() {
    section("Lesson 9: Traits");

    let dog = Dog { name: String::from("Rex") };
    let cat = Cat { name: String::from("Mia") };

    // Traits define shared behavior. Any type can implement them.
    println!("  dog says: {}", dog.speak());
    println!("  cat says: {}", cat.speak());

    // Default trait methods can be used or overridden.
    println!("  dog info: {}", dog.description());
    println!("  cat info: {}", cat.description());

    // Trait bounds restrict generic code to types that implement a trait.
    print_animals(&[&dog, &cat]);

    // `impl Trait` in parameters is syntax sugar for a trait bound.
    let area = rectangle_area(Rectangle { width: 4.0, height: 3.0 });
    println!("  rectangle area: {area:.1}");

    println!();
    println!("  Notes:");
    println!("  - traits define shared behavior");
    println!("  - impl Trait adds methods to a type");
    println!("  - trait bounds limit which types a generic accepts");
}

/// Shared behavior for animals that can speak.
trait Animal {
    fn name(&self) -> &str;

    /// Default implementation. Types can override this.
    fn speak(&self) -> String {
        String::from("...")
    }

    fn description(&self) -> String {
        format!("{} ({})", self.name(), self.speak())
    }
}

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }

    fn speak(&self) -> String {
        String::from("woof")
    }
}

impl Animal for Cat {
    fn name(&self) -> &str {
        &self.name
    }

    fn speak(&self) -> String {
        String::from("meow")
    }
}

/// Trait bound: T must implement Animal.
fn print_animals(animals: &[&dyn Animal]) {
    print!("  animals: ");
    for animal in animals {
        print!("{} ", animal.name());
    }
    println!();
}

/// Trait for shapes that have a calculable area.
trait Area {
    fn area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

/// `impl Area` means: accept any type that implements Area.
fn rectangle_area(shape: impl Area) -> f64 {
    shape.area()
}
