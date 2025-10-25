use std::io;

fn main() {
    let mut experience = String::new();
    let mut age = String::new();

    println!("Is the employee experienced? (yes/no):");
    io::stdin().read_line(&mut experience).unwrap();

    println!("Enter the employee's age:");
    io::stdin().read_line(&mut age).unwrap();

    let experienced = experience.trim().eq_ignore_ascii_case("yes");
    let age: u32 = age.trim().parse().unwrap();

    if experienced && age >= 40 {
        println!("Incentive: N1,560,000");
    } else if experienced && age >= 30 {
        println!("Incentive: N1,480,000");
    } else if experienced && age < 28 {
        println!("Incentive: N1,300,000");
    } else {
        println!("Incentive: N100,000");
    }
}