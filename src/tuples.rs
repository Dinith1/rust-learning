// Tuples group together values of different types
// Max 12 elements

pub fn run() {
    let person: (&str, &str, i8) = ("Dinith", "Canva", 23);

    println!(
        "{} is {} years old and works at {}",
        person.0, person.2, person.1
    );
}
