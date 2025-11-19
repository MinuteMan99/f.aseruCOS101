fn main() {
    // Using Vec::new() to create a new, empty vector
    let v: Vec<i32> = Vec::new();

    // printing the size of vector
    println!("\nThe length of Vec::new is: {}", v.len());

    // Using the vec! macro for creating a vector with initial values
    let v = vec!["Grace", "Effiong", "Basil", "Kareem", "Susan"];

    // Printing the size of the vector created with the macro
    println!("\nThe length of vec macro is: {}", v.len());
}