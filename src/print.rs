pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // Basic formatting
    println!("Name: {}, Number: {}", "Dinith", 1);

    // Positional arguments
    println!("{0} {2} {1} {3}", "1st", "2nd", "3rd", "4th");

    // Named arguments
    println!("{age} {name}", name = "Dinith", age = "23");

    // Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait (print multiple values)
    println!("{:?}", (12, true, "hello"));

    // Math
    println!("10 + 10 = {}", 10 + 10);
}
