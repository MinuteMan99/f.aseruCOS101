use std::io::{self, Write};
use std::f64::consts::PI;

// --- INPUT HANDLERS ---

/// Reads a line from standard input, trims it, and returns it as a String.
fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

/// Prompts the user for input and attempts to parse it as an f64.
/// It loops until valid input is received.
fn read_f64_input(prompt: &str) -> f64 {
    loop {
        print!("{}: ", prompt);
        // Ensure the prompt is immediately displayed
        io::stdout().flush().expect("Failed to flush stdout");

        let input = read_input();
        match input.parse::<f64>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
            }
        }
    }
}

// --- CALCULATION FUNCTIONS ---

/// Calculates the area of a Trapezium.
/// Formula: height * 0.5 * (base1 + base2)
fn calculate_trapezium_area() {
    println!("\n--- Area of Trapezium ---");
    let height = read_f64_input("Enter height (h)");
    let base1 = read_f64_input("Enter first base (b1)");
    let base2 = read_f64_input("Enter second base (b2)");

    let area = 0.5 * height * (base1 + base2);
    println!("\nThe Area of the Trapezium is: {:.2}", area);
}

/// Calculates the area of a Rhombus.
/// Formula: 0.5 * diagonal1 * diagonal2
fn calculate_rhombus_area() {
    println!("\n--- Area of Rhombus ---");
    let d1 = read_f64_input("Enter length of diagonal 1 (d1)");
    let d2 = read_f64_input("Enter length of diagonal 2 (d2)");

    let area = 0.5 * d1 * d2;
    println!("\nThe Area of the Rhombus is: {:.2}", area);
}

/// Calculates the area of a Parallelogram.
/// Formula: base * altitude
fn calculate_parallelogram_area() {
    println!("\n--- Area of Parallelogram ---");
    let base = read_f64_input("Enter base (b)");
    let altitude = read_f64_input("Enter altitude (a)");

    let area = base * altitude;
    println!("\nThe Area of the Parallelogram is: {:.2}", area);
}

/// Calculates the surface area of a Cube (as per the assignment text "Area of Cube").
/// Formula: 6 * (length of the side)^2
fn calculate_cube_surface_area() {
    println!("\n--- Surface Area of Cube ---");
    let side = read_f64_input("Enter length of the side (s)");

    let area = 6.0 * side.powi(2);
    println!("\nThe Surface Area of the Cube is: {:.2}", area);
}

/// Calculates the Volume of a Cylinder.
/// Formula: π * radius^2 * height
fn calculate_cylinder_volume() {
    println!("\n--- Volume of Cylinder ---");
    let radius = read_f64_input("Enter radius (r)");
    let height = read_f64_input("Enter height (h)");

    let volume = PI * radius.powi(2) * height;
    println!("\nThe Volume of the Cylinder is: {:.2}", volume);
}


/// The main entry point of the program. Displays the menu and handles choice.
fn main() {
    println!("Welcome to the MTH 101 Geometric Calculator!");

    loop {
        // Display the menu
        println!("\n==============================================");
        println!("Please select a calculation by entering the corresponding number:");
        println!("1. Area of Trapezium");
        println!("2. Area of Rhombus");
        println!("3. Area of Parallelogram");
        println!("4. Surface Area of Cube (as 'Area of Cube' in assignment)");
        println!("5. Volume of Cylinder");
        println!("6. Exit Program");
        println!("==============================================");

        print!("Your choice: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let choice = read_input();

        match choice.as_str() {
            "1" => calculate_trapezium_area(),
            "2" => calculate_rhombus_area(),
            "3" => calculate_parallelogram_area(),
            "4" => calculate_cube_surface_area(),
            "5" => calculate_cylinder_volume(),
            "6" => {
                println!("\nThank you for using the calculator. Goodbye!");
                break; // Exit the loop and end the program
            }
            _ => {
                println!("\nError: Invalid selection. Please enter a number between 1 and 6.");
            }
        }
    }
}