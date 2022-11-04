pub fn run() {
    let age: u8 = 22;
    let check_id: bool = false;
    let know_person_of_age = true;

    if age >= 21 && check_id || know_person_of_age {
        println!("Basrtender: What would like to drink?")
    } else if age < 21 && check_id {
        println!("Basrtender: Sorry, you have to leave")
    } else {
        println!("Basrtender: I'll need to see your id")
    }

    let is_of_age = if age >= 21 { true } else { false };

    println!("is Of Age: {}", is_of_age);
}
