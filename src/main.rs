use std::str::FromStr;

mod advanced;
mod beginner;

fn main() {
    println!("Hello, world!");

    // beginner module

    // beginner::formatting::run();
    // beginner::vars::run();
    // beginner::types::run();
    // beginner::strings::run();
    // beginner::tuples::run();
    // beginner::arrays::run();
    // beginner::vectors::run();
    // beginner::conditionals::run();
    // beginner::loops::run();
    // beginner::functions::run();
    // beginner::pointer_ref::run();
    // beginner::structs::run();
    // beginner::enums::run();
    // beginner::cli::run();

    // advanced

    // advanced::traits::run();

    let file_path = String::from("test.txt");
    advanced::io::run(file_path);
}
