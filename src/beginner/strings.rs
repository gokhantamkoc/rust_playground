// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when manipulating a string value

pub fn run() {
    let hello = "Hello"; // Immutable
    let mut hello2 = String::from("Hello"); //Growable

    // Get Length
    println!("Length: {}", hello.len());
    println!("Length: {}", hello2.len());

    // Push string
    hello2.push_str(" W");
    hello2.push_str("orld!");
    println!("{}", hello2);

    // Capacity in bytes
    println!("Capacity: {} bytes", hello2.capacity());

    // Contains
    println!("Contains World: {}", hello2.contains("World"));

    // Loop through string by whitespace
    for word in hello2.split_whitespace() {
        println!("{}", word);
    }

    // Create String with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('a');

    println!("{}", s);

    // Assertion testing
    assert_eq!(5, s.len());
}
