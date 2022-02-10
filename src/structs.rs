// Structs - Used to create custom data types

// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple Struct
struct ColorT(u8, u8, u8);

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 255,
        blue: 255,
    };

    c.red = 200;
    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut c2 = ColorT(243, 221, 23);
    c2.2 = 100;
    println!("ColorT: {} {} {}", c2.0, c2.1, c2.2);

    struct Person {
        first_name: String,
        last_name: String,
    }

    impl Person {
        // Construct a Person
        fn new(first: &str, last: &str) -> Person {
            Person {
                first_name: first.to_string(),
                last_name: last.to_string(),
            }
        }

        // Get fullname
        fn full_name(&self) -> String {
            format!("{} {}", self.first_name, self.last_name)
        }

        // Set last_name
        fn set_last_name(&mut self, _last_name: &str) {
            self.last_name = _last_name.to_string();
        }

        // To Tuple
        fn to_tuple(self) -> (String, String) {
            (self.first_name, self.last_name)
        }
    }

    let mut p = Person::new("Gurbet", "Arslan");
    println!("Person {} {}", p.first_name, p.last_name);
    p.set_last_name("Tamkoç");
    println!("Married Person {}", p.full_name());
    println!("Married Person {:?}", p.to_tuple());
}
