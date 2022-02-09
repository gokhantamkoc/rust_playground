// Loops - Used to iterate until a condition is met

pub fn run() {
    let mut count = 0;

    // Infinite loop
    loop {
        count += 1;
        println!("Loop: {}", count);
        if count == 10 {
            break;
        }
    }

    // While loop
    while count < 20 {
        count += 1;
        println!("While: {}", count);
    }

    // For loop
    for x in 0..100 {
        println!("For: {}", x);
    }
}