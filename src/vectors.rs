// Vectors - Resizable arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // Get single val
    println!("Value at 0: {}", numbers[0]);

    // Get vector length
    println!("Length: {}", numbers.len());

    // Vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..3];
    println!("Slice {:?}", slice);
    

    // Add to vector
    numbers.push(25);
    numbers.push(52);

    // Remove last added value from vector
    numbers.pop();
    println!("vector {:?}", numbers);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop & mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Current vector {:?}", numbers);
}