pub fn run() {
    let age = 18;
    let check_id: bool = false;
    let person_age_known = true;

    if check_id || person_age_known {
        if age >= 21 {
            println!("Bartender: What would you like to drink?");
        } else if age < 21 {
            println!("Bartender: Sorry! you have to leave.");
        }
    } else {
        println!("Bartender: I need to see your ID.");
    }

    // Shorthand If
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of age: {}", is_of_age);
}