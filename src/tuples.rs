// Tuples group together values of different types
// Max 12 elements

pub fn run() {
    let person: (&str, &str, i8) = ("Gökhan", "Çınar", 37);
    println!(
        "{} has a son called {} and he is {}",
        person.0, person.1, person.2
    );
}
