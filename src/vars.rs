// Variables hold primitives or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    // Not mutable
    let name = "Dinith";
    // Mutable
    let mut age = 23;
    println!("My name is {} and I am {}", name, age);
    age = 24;
    println!("Now I'm {}", age);

    // Define constants - use uppercase + define the type
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Dinith", 23);
    println!("{} {}", my_name, my_age);
}
