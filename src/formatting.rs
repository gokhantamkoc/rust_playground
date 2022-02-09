pub fn run() {

    // Print to console
    println!("Hello from print.rs file");

    // Basic formatting
    println!("{} tests {}.", "Gökhan", "the code");

    // Positional formatting
    println!("{0} tests {1} because he thinks {1} should be tested.", "Gökhan", "the code");

    // Named formatting
    println!("{name} likes to play {activity}", name="Gökhan", activity="games");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);
    
    // Placeholder traits with position
    println!("Binary: {0:b} Hex: {0:x} Octal: {0:o}", 10);
    
    // Placeholder for debug trait
    println!("{:?}", (12, true, "Gökhan"));

    // Basic Math
    println!("10 + 10 = {}", 10 + 10);
}