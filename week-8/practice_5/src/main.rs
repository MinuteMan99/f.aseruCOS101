fn main() {
    // Create an empty vector "City"
    let mut city: Vec<String> = Vec::new();
    // Print City vector 
    println!("\nThe City vector has element(s) {}",city.len());
    // Push new elements
    let mut input1 = String::new();
    println!("How many Cities do you want to enter");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let city_num:i32 = input1.trim().parse().expect("Invalid input");
    for count in 0..city_num {
        let mut input2 = String::new();
        println!("Enter City {}:", count);
        std::io::stdin().read_line(&mut input2).expect("Failed to read input");
        let new_city = input2.trim().to_string();
        city.push(new_city);
    }
    println!("\nYour preferred cities are:\n");
    let mut count=1;
    //loop to iterate elements in vector
    for i in city.iter() {
        // iterating through c on the vector
        println!("City {}: {}\n", count, i);
        count += 1;
    }
}