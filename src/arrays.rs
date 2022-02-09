// Arrays - Fixed list where elements are the same data types

use std::mem;

pub fn run() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // Get single val
    println!("Value at 0: {}", numbers[0]);

    // Mutable arrays
    let mut mut_numbers: [i32; 4] = [1, 2, 3, 4];

    // Get array length
    println!("Length: {}", mut_numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&mut_numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..3];
    println!("Slice {:?}", slice);
}