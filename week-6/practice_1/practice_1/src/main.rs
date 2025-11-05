fn main() {
    let name = "Aisha Lawal";
    let uni:&'static str = "Pan-Atlantic University";
    let addr:&str = "Km 52 Lekki-Epe Expressway, Ibeju-Lekki, Lagos";
    println!("Name: {}", name);
    println!("University: {}", uni);
    println!("Address: {}", addr);

    let department:&'static str = "Computer Science";
    let school:&'static str = "School of Science and Technology";
    println!("Department: {} School: {}", department, school);
}