// Structs - Used to create custom data types

// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
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
}