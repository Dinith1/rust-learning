// Vectors are resizable arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];
    println!("{:?}", numbers);

    numbers[3] = 500;
    println!("{:?}", numbers);

    // Vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice (don't have to specify type of the var)
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // Add to vector
    numbers.push(600);
    numbers.push(15);
    println!("{:?}", numbers);

    // Remove elements
    numbers.pop();
    println!("{:?}", numbers);
    numbers.remove(2);
    println!("{:?}", numbers);

    // Loop
    for x in numbers.iter_mut() {
        println!("{}", x);
        *x *= 2;
    }

    println!("{:?}", numbers);
}
