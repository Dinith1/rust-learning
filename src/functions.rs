pub fn run() {
    greeting("Hello", "Dinith");
    let sum = add(10, 15);
    println!("Sum: {}", sum);

    let n3: i32 = 10;
    // Closure
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure sum: {}", add_nums(3, 12));
}

fn greeting(greet: &str, name: &str) {
    println!("{}, {}. Nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    // No semi-colon on the end of line = return
    n1 + n2
}
