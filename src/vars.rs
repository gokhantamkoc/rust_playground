// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Gökhan";
    let mut age = 32;           // let variables are immutable by default. you need to put mut.
    age = 21;

    println!("My name is {} and I am {}.", name, age);

    // define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // multiple variables
    let (my_name, my_age) = ("Gökhan", 32);
    println!("{} is {}", my_name, my_age);
}