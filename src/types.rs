/*
Rust is statically typed, but it can infer the type from the assigning.

Primitive types:
    - Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
    - Floats: f32, f64
    - Boolean: bool
    - Characters: char
    - Tuples
    - Arrays
*/

pub fn run() {
    // Default int is i32
    let x = 1;

    // Default float is f64
    let y = 2.5;

    // Add explicit type
    let a: i64 = 454545454554;
    let b: f32 = 1235.8;

    println!("{} {} {} {}", x, y, a, b);

    // Find min/max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
    println!("Min f64: {}", std::f64::MIN);

    // Boolean
    let is_active = true;
    let is_cool = x > a;

    // Char
    let a1 = 'a';
    let face = '\u{1F600}'; // Emoji unicode

    println!("{:?}", (x, y, is_active, is_cool, a1, face));
}
