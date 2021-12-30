// Arrays are fixed lists where elements are the same data types

use std::mem;

pub fn run() {
    // Immutable
    let numbers: [i32; 4] = [1, 2, 3, 4];
    println!("{:?}", numbers);

    // Get single value
    println!("{}", numbers[0]);

    let mut mutable_numbers = [5, 10, 15, 20];
    mutable_numbers[3] = 500;
    println!("{:?}", mutable_numbers);

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice (don't have to specify type of the var)
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}
