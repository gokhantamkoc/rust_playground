use std::fs;

pub fn run(file_path: String) {
    // --snip--
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
