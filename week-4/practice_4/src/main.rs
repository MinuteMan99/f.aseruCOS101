use std::io;

fn main() {
    let mut name = String::new();
    let mut age_str = String::new();

    println!("Enter your name:");
    io::stdin().read_line(&mut name).expect("Not a valid string");

    println!("Enter your age:");
    io::stdin().read_line(&mut age_str).expect("Not a valid string");
    let age: i32 = age_str.trim().parse().expect("Not a valid number");

    if age >= 18 {
        println!("Welcome to the party, {}!", name.trim());
    } else {
        println!("Oops, you are not of age to enter the party, {}.", name.trim());
    }
}
