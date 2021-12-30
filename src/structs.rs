// Structs - used to create custom data types

// Traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple struct
struct Color2(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

// Functions for Person struct
impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // Get full name - '&' used because the method 'borrows' self - copies value into new memory for the String
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple - No '&' used because the method 'consumes' self - tuple now references the first/last_name strings (the struct does not)
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    // Traditional struct
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };
    println!("Color: {} {} {}", c.red, c.green, c.blue);
    c.red = 100;
    println!("Color: {} {} {}", c.red, c.green, c.blue);

    // Tuple struct
    let mut c2 = Color2(0, 56, 123);
    println!("Color: {} {} {}", c2.0, c2.1, c2.2);
    c2.0 = 100;
    c2.1 = 42;
    println!("Color: {} {} {}", c2.0, c2.1, c2.2);

    // Traditional struct with functions
    let mut p = Person::new("John", "Smith");
    println!("{} {}", p.first_name, p.last_name);
    println!("{}", p.full_name());
    p.set_last_name("Doe");
    println!("{}", p.full_name());
    println!("Tuple: {:?}", p.to_tuple()); // Variable p no longer exists*
}
