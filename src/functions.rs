pub fn run() {
    greeting("Hello", "Çınar");

    // Bind function values to variables
    let sum = add(1, 2);
    println!("Sum: {}", sum);

    // Closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure res: {}", add_nums(4,6));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}