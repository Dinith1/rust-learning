// Primitive str = Immutable, fixed-length somewhere in memory
// String = growable, heap-allocated data structure

pub fn run() {
    // Fixed length (but can still change entire string if mut keyword used)
    let mut str1 = "Hello";
    println!("{} {}", str1, str1.len());
    str1 = "HELLLLLOOOOO";
    println!("{} {}", str1, str1.len());

    // Variable length
    let mut str2 = String::from("Hello");
    println!("{}", str2);
    // Push char
    str2.push('A');
    println!("{}", str2);
    // Push str
    str2.push_str(" BBBB");
    println!("{}", str2);
    str2 = str2 + " CCC";
    println!("{}", str2);

    // Capacity in bytes + some other methods
    println!(
        "Capacities: {:?}",
        (str2.capacity(), str2.is_empty(), str2.contains("BB"))
    );

    // Replace
    str2 = str2.replace("Hello", "Goodbye");
    println!("{}", str2);

    // Loop through string by whitespace
    for word in str2.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing - throws error if not true
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);
}
